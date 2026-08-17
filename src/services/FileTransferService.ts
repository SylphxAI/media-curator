import { ExifTool } from 'exiftool-vendored';
// import { injectable } from "inversify"; // Removed unused 'inject' - REMOVED INVERSIFY
import { join, basename, extname, parse, normalize, resolve } from 'path'; // Added normalize
import { existsSync } from 'fs';
import crypto from 'crypto';
import chalk from 'chalk';
import { MultiBar, Presets } from 'cli-progress';
import {
  FileInfo,
  DeduplicationResult,
  GatherFileInfoResult,
  FileProcessorConfig,
} from '../types';
// import { MediaProcessor } from "../MediaProcessor"; // REMOVED - Replaced by processSingleFile function
import { processSingleFile } from '../fileProcessor';
import { AppResult } from '../errors'; // Import AppResult for return type handling
import { LmdbCache } from '../caching/LmdbCache';
import { WorkerPool } from '../contexts/types';
import { transferOrCopyFile as executeTransfer } from '../transferOps.js';
import {
  fingerprintFile,
  type OrganizationPlanAction,
  type OrganizationPlanActionKind,
  type OrganizationPlanFingerprint,
} from '../organizationPlan.js';

interface TransferAction {
  kind: OrganizationPlanActionKind;
  source: string;
  target: string;
  reason: string;
  representative?: string;
  fingerprint?: OrganizationPlanFingerprint;
}

// @injectable() // REMOVED INVERSIFY
export class FileTransferService {
  constructor(
    // Manually injected dependencies for processSingleFile
    private readonly config: FileProcessorConfig,
    private readonly cache: LmdbCache,
    private readonly exifTool: ExifTool,
    private readonly workerPool: WorkerPool,
  ) {}

  async transferOrganizedFiles(
    gatherFileInfoResult: GatherFileInfoResult,
    deduplicationResult: DeduplicationResult,
    targetDir: string,
    duplicateDir: string | undefined,
    errorDir: string | undefined,
    format: string,
    shouldMove: boolean,
  ): Promise<void> {
    const actions = await this.collectTransferActions(
      gatherFileInfoResult,
      deduplicationResult,
      targetDir,
      duplicateDir,
      errorDir,
      format,
      false,
    );
    const multibar = new MultiBar(
      {
        clearOnComplete: false,
        hideCursor: true,
        format:
          '{phase} ' +
          chalk.cyan('{bar}') +
          ' {percentage}% || {value}/{total} Files',
      },
      Presets.shades_classic,
    );
    const transfer = async (
      sourcePath: string,
      targetPath: string,
      isCopy: boolean,
    ): Promise<void> => {
      try {
        await this.transferOrCopyFile(sourcePath, targetPath, isCopy);
      } catch (error) {
        multibar.stop();
        throw error;
      }
    };

    const bars = {
      organize: multibar.create(
        actions.filter((action) => action.kind === 'organize').length,
        0,
        { phase: 'Unique  ' },
      ),
      duplicate: multibar.create(
        actions.filter((action) => action.kind === 'duplicate').length,
        0,
        { phase: 'Duplicate' },
      ),
      error: multibar.create(
        actions.filter((action) => action.kind === 'error').length,
        0,
        { phase: 'Error   ' },
      ),
    };

    for (const action of actions) {
      await transfer(action.source, action.target, !shouldMove);
      bars[action.kind].increment();
    }

    const duplicateCount = actions.filter(
      (action) => action.kind === 'duplicate',
    ).length;
    if (duplicateDir && duplicateCount > 0) {
      console.log(
        chalk.yellow(
          `\nDuplicate files have been ${shouldMove ? 'moved' : 'copied'} to ${duplicateDir}`,
        ),
      );
    }
    if (duplicateDir && duplicateCount === 0) {
      console.log(chalk.yellow('\nNo duplicate files to process.'));
    }
    const errorCount = actions.filter(
      (action) => action.kind === 'error',
    ).length;
    if (errorDir && errorCount > 0) {
      console.log(
        chalk.red(
          `\nError files have been ${shouldMove ? 'moved' : 'copied'} to ${errorDir}`,
        ),
      );
    }

    multibar.stop();
    console.log(chalk.green('\nFile transfer completed'));
  }

  async planOrganizedFiles(
    gatherFileInfoResult: GatherFileInfoResult,
    deduplicationResult: DeduplicationResult,
    targetDir: string,
    duplicateDir: string | undefined,
    errorDir: string | undefined,
    format: string,
  ): Promise<OrganizationPlanAction[]> {
    const actions = await this.collectTransferActions(
      gatherFileInfoResult,
      deduplicationResult,
      targetDir,
      duplicateDir,
      errorDir,
      format,
      true,
    );
    return actions.map((action, index) => {
      if (!action.fingerprint) {
        throw new Error(`Missing fingerprint for planned action ${index + 1}`);
      }
      const plannedAction: OrganizationPlanAction = {
        id: `${action.kind}-${String(index + 1).padStart(6, '0')}`,
        kind: action.kind,
        source: resolve(action.source),
        target: resolve(action.target),
        reason: action.reason,
        fingerprint: action.fingerprint,
      };
      if (action.representative) {
        plannedAction.representative = resolve(action.representative);
      }
      return plannedAction;
    });
  }

  private async collectTransferActions(
    gatherFileInfoResult: GatherFileInfoResult,
    deduplicationResult: DeduplicationResult,
    targetDir: string,
    duplicateDir: string | undefined,
    errorDir: string | undefined,
    format: string,
    includeFingerprints: boolean,
  ): Promise<TransferAction[]> {
    const actions: TransferAction[] = [];
    const addAction = async (
      action: Omit<TransferAction, 'fingerprint'>,
    ): Promise<void> => {
      const fingerprint = includeFingerprints
        ? await fingerprintFile(action.source)
        : undefined;
      const transferAction: TransferAction = { ...action };
      if (fingerprint) transferAction.fingerprint = fingerprint;
      actions.push(transferAction);
    };

    for (const filePath of [...deduplicationResult.uniqueFiles].sort()) {
      const fileInfoResult: AppResult<FileInfo> = await processSingleFile(
        filePath,
        this.config,
        this.cache,
        this.exifTool,
        this.workerPool,
      );
      if (fileInfoResult.isErr()) {
        throw fileInfoResult.error;
      }
      await addAction({
        kind: 'organize',
        source: filePath,
        target: this.generateTargetPath(
          format,
          targetDir,
          fileInfoResult.value,
          filePath,
        ),
        reason: 'selected representative for the organized library',
      });
    }

    if (duplicateDir) {
      const duplicateSets = [...deduplicationResult.duplicateSets].sort(
        (a, b) => String(a.bestFile).localeCompare(String(b.bestFile)),
      );
      for (const duplicateSet of duplicateSets) {
        const bestFile = String(duplicateSet.bestFile);
        const duplicateFolderName = basename(bestFile, extname(bestFile));
        const duplicateSetFolder = join(duplicateDir, duplicateFolderName);
        for (const duplicatePath of [...duplicateSet.duplicates].sort()) {
          await addAction({
            kind: 'duplicate',
            source: duplicatePath,
            target: join(duplicateSetFolder, basename(duplicatePath)),
            reason: `duplicate of representative ${bestFile}`,
            representative: bestFile,
          });
        }
      }
    }

    if (errorDir) {
      for (const errorFilePath of [...gatherFileInfoResult.errorFiles].sort()) {
        await addAction({
          kind: 'error',
          source: errorFilePath,
          target: join(errorDir, basename(errorFilePath)),
          reason: 'processing error during library admission',
        });
      }
    }

    return actions;
  }

  private async transferOrCopyFile(
    sourcePath: string,
    targetPath: string,
    isCopy: boolean,
  ): Promise<void> {
    await executeTransfer(sourcePath, targetPath, isCopy);
  }

  private generateTargetPath(
    format: string,
    targetDir: string,
    fileInfo: FileInfo,
    sourcePath: string,
  ): string {
    const mixedDate =
      fileInfo.metadata.imageDate || fileInfo.fileStats.createdAt;
    const { name, ext } = parse(sourcePath);

    // Moved generateRandomId to be a private method of the class

    const data: { [key: string]: string } = {
      'I.YYYY': this.formatDate(fileInfo.metadata.imageDate, 'YYYY'),
      'I.YY': this.formatDate(fileInfo.metadata.imageDate, 'YY'),
      'I.MMMM': this.formatDate(fileInfo.metadata.imageDate, 'MMMM'),
      'I.MMM': this.formatDate(fileInfo.metadata.imageDate, 'MMM'),
      'I.MM': this.formatDate(fileInfo.metadata.imageDate, 'MM'),
      'I.M': this.formatDate(fileInfo.metadata.imageDate, 'M'),
      'I.DD': this.formatDate(fileInfo.metadata.imageDate, 'DD'),
      'I.D': this.formatDate(fileInfo.metadata.imageDate, 'D'),
      'I.DDDD': this.formatDate(fileInfo.metadata.imageDate, 'DDDD'),
      'I.DDD': this.formatDate(fileInfo.metadata.imageDate, 'DDD'),
      'I.HH': this.formatDate(fileInfo.metadata.imageDate, 'HH'),
      'I.H': this.formatDate(fileInfo.metadata.imageDate, 'H'),
      'I.hh': this.formatDate(fileInfo.metadata.imageDate, 'hh'),
      'I.h': this.formatDate(fileInfo.metadata.imageDate, 'h'),
      'I.mm': this.formatDate(fileInfo.metadata.imageDate, 'mm'),
      'I.m': this.formatDate(fileInfo.metadata.imageDate, 'm'),
      'I.ss': this.formatDate(fileInfo.metadata.imageDate, 'ss'),
      'I.s': this.formatDate(fileInfo.metadata.imageDate, 's'),
      'I.a': this.formatDate(fileInfo.metadata.imageDate, 'a'),
      'I.A': this.formatDate(fileInfo.metadata.imageDate, 'A'),
      'I.WW': this.formatDate(fileInfo.metadata.imageDate, 'WW'),

      'F.YYYY': this.formatDate(fileInfo.fileStats.createdAt, 'YYYY'),
      'F.YY': this.formatDate(fileInfo.fileStats.createdAt, 'YY'),
      'F.MMMM': this.formatDate(fileInfo.fileStats.createdAt, 'MMMM'),
      'F.MMM': this.formatDate(fileInfo.fileStats.createdAt, 'MMM'),
      'F.MM': this.formatDate(fileInfo.fileStats.createdAt, 'MM'),
      'F.M': this.formatDate(fileInfo.fileStats.createdAt, 'M'),
      'F.DD': this.formatDate(fileInfo.fileStats.createdAt, 'DD'),
      'F.D': this.formatDate(fileInfo.fileStats.createdAt, 'D'),
      'F.DDDD': this.formatDate(fileInfo.fileStats.createdAt, 'DDDD'),
      'F.DDD': this.formatDate(fileInfo.fileStats.createdAt, 'DDD'),
      'F.HH': this.formatDate(fileInfo.fileStats.createdAt, 'HH'),
      'F.H': this.formatDate(fileInfo.fileStats.createdAt, 'H'),
      'F.hh': this.formatDate(fileInfo.fileStats.createdAt, 'hh'),
      'F.h': this.formatDate(fileInfo.fileStats.createdAt, 'h'),
      'F.mm': this.formatDate(fileInfo.fileStats.createdAt, 'mm'),
      'F.m': this.formatDate(fileInfo.fileStats.createdAt, 'm'),
      'F.ss': this.formatDate(fileInfo.fileStats.createdAt, 'ss'),
      'F.s': this.formatDate(fileInfo.fileStats.createdAt, 's'),
      'F.a': this.formatDate(fileInfo.fileStats.createdAt, 'a'),
      'F.A': this.formatDate(fileInfo.fileStats.createdAt, 'A'),
      'F.WW': this.formatDate(fileInfo.fileStats.createdAt, 'WW'),

      'D.YYYY': this.formatDate(mixedDate, 'YYYY'),
      'D.YY': this.formatDate(mixedDate, 'YY'),
      'D.MMMM': this.formatDate(mixedDate, 'MMMM'),
      'D.MMM': this.formatDate(mixedDate, 'MMM'),
      'D.MM': this.formatDate(mixedDate, 'MM'),
      'D.M': this.formatDate(mixedDate, 'M'),
      'D.DD': this.formatDate(mixedDate, 'DD'),
      'D.D': this.formatDate(mixedDate, 'D'),
      'D.DDDD': this.formatDate(mixedDate, 'DDDD'),
      'D.DDD': this.formatDate(mixedDate, 'DDD'),
      'D.HH': this.formatDate(mixedDate, 'HH'),
      'D.H': this.formatDate(mixedDate, 'H'),
      'D.hh': this.formatDate(mixedDate, 'hh'),
      'D.h': this.formatDate(mixedDate, 'h'),
      'D.mm': this.formatDate(mixedDate, 'mm'),
      'D.m': this.formatDate(mixedDate, 'm'),
      'D.ss': this.formatDate(mixedDate, 'ss'),
      'D.s': this.formatDate(mixedDate, 's'),
      'D.a': this.formatDate(mixedDate, 'a'),
      'D.A': this.formatDate(mixedDate, 'A'),
      'D.WW': this.formatDate(mixedDate, 'WW'),

      NAME: name,
      'NAME.L': name.toLowerCase(),
      'NAME.U': name.toUpperCase(),
      EXT: ext.slice(1).toLowerCase(), // Put EXT back in data
      RND: this.generateRandomId(), // Call as a method
      GEO:
        fileInfo.metadata.gpsLatitude && fileInfo.metadata.gpsLongitude
          ? `${fileInfo.metadata.gpsLatitude.toFixed(2)}_${fileInfo.metadata.gpsLongitude.toFixed(2)}`
          : '',
      CAM: fileInfo.metadata.cameraModel || '',
      TYPE: fileInfo.media.duration > 0 ? 'Video' : 'Image',
      'HAS.GEO':
        fileInfo.metadata.gpsLatitude && fileInfo.metadata.gpsLongitude
          ? 'GeoTagged'
          : 'NoGeo',
      'HAS.CAM': fileInfo.metadata.cameraModel ? 'WithCamera' : 'NoCamera',
      'HAS.DATE':
        fileInfo.metadata.imageDate &&
        !isNaN(fileInfo.metadata.imageDate.getTime())
          ? 'Dated'
          : 'NoDate',
    };

    // Build a regex that specifically matches the known format keys from the 'data' object
    const knownKeys = Object.keys(data).map((key) =>
      key.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'),
    ); // Escape regex special chars in keys
    // Sort keys by length descending to match longer keys first (e.g., NAME.L before NAME) - might not be strictly necessary here but good practice
    knownKeys.sort((a, b) => b.length - a.length);
    // Regex to match {KEY}
    const formatRegex = new RegExp(`\\{(${knownKeys.join('|')})\\}`, 'g');

    const originalExt = parse(sourcePath).ext; // Get original extension with dot (e.g., ".jpg")

    let formattedPath = format.replace(formatRegex, (match, key) => {
      let replacement = '';
      if (key === 'EXT') {
        // Use the original extension *with* the dot if the key is EXT
        replacement = originalExt;
      } else {
        replacement = data[key] || ''; // Get value from data object for other keys
      }
      // Sanitize the replacement value (important for NAME, CAM etc.)
      // Don't sanitize the extension itself here.
      if (key !== 'EXT') {
        replacement = replacement.replace(/[<>:"|?*]/g, '_');
      }
      return replacement;
    });

    // Removed the redundant originalExt definition and the separate replace call for EXT

    // Remove leading/trailing slashes and ensure single slashes
    formattedPath = formattedPath
      .split(/[/\\]+/)
      .filter(Boolean)
      .join('/'); // Removed unnecessary escape for /

    if (!formattedPath) {
      formattedPath = 'NoDate'; // Default folder if format string results in empty path
    }

    // Determine if the format string likely intended to specify a filename
    const formatSpecifiesFilename =
      /\{NAME/.test(format) || /\{EXT\}/.test(format); // Simple check

    let directory: string;
    let finalFilenameBase: string;
    let finalFilenameExt: string;

    if (formatSpecifiesFilename) {
      // Assume formattedPath contains the intended directory and base filename (potentially without ext)
      const parsedFormatted = parse(formattedPath.replace(/\\/g, '/'));
      directory = parsedFormatted.dir;
      finalFilenameBase = parsedFormatted.name; // Name part from format
      // Use extension from format if present, otherwise use original
      finalFilenameExt = parsedFormatted.ext || originalExt;
    } else {
      // Format string only specified directory structure
      directory = formattedPath; // The whole thing is the directory
      finalFilenameBase = name; // Use original base name
      finalFilenameExt = ext; // Use original extension
    }

    // Combine base and extension
    let finalFilename = `${finalFilenameBase}${finalFilenameExt}`;

    // Sanitize filename part as well
    finalFilename = finalFilename.replace(/[<>:"/\\|?*]/g, '_');

    let fullPath = join(targetDir, directory, finalFilename); // Already correct here, but included for context

    // Handle potential filename conflicts
    let counter = 1;
    // Parse the *sanitized* filename to get parts for conflict resolution
    const parsedSanitizedFilename = parse(finalFilename);
    const baseNameForConflict = parsedSanitizedFilename.name;
    const extensionForConflict = parsedSanitizedFilename.ext;

    // Normalize the path *before* checking existence in the loop
    while (existsSync(normalize(fullPath))) {
      // Option 1: Append counter
      // filename = `${parsedFilename.name}_${counter++}${parsedFilename.ext}`;

      // Option 2: Append random ID (as was done before, but maybe only on conflict)
      finalFilename = `${baseNameForConflict}_${this.generateRandomId()}${extensionForConflict}`; // Call as a method

      fullPath = join(targetDir, directory, finalFilename); // Already correct here, but included for context
      // Safety break to prevent infinite loops in weird edge cases
      if (counter > 100) {
        console.error(
          chalk.red(
            `Could not resolve filename conflict for ${sourcePath} after 100 attempts.`,
          ),
        );
        throw new Error(
          `Filename conflict resolution failed for ${sourcePath}`,
        );
      }
      counter++;
    }

    return fullPath;
  }

  private formatDate(date: Date | undefined, format: string): string {
    if (!date || isNaN(date.getTime())) {
      return '';
    }

    const pad = (num: number) => num.toString().padStart(2, '0');

    const formatters: { [key: string]: () => string } = {
      YYYY: () => date.getFullYear().toString(),
      YY: () => date.getFullYear().toString().slice(-2),
      MMMM: () => date.toLocaleString('default', { month: 'long' }),
      MMM: () => date.toLocaleString('default', { month: 'short' }),
      MM: () => pad(date.getMonth() + 1),
      M: () => (date.getMonth() + 1).toString(),
      DD: () => pad(date.getDate()),
      D: () => date.getDate().toString(),
      DDDD: () => date.toLocaleString('default', { weekday: 'long' }),
      DDD: () => date.toLocaleString('default', { weekday: 'short' }),
      HH: () => pad(date.getHours()),
      H: () => date.getHours().toString(),
      hh: () => pad(date.getHours() % 12 || 12),
      h: () => (date.getHours() % 12 || 12).toString(),
      mm: () => pad(date.getMinutes()),
      m: () => date.getMinutes().toString(),
      ss: () => pad(date.getSeconds()),
      s: () => date.getSeconds().toString(),
      a: () => (date.getHours() < 12 ? 'am' : 'pm'),
      A: () => (date.getHours() < 12 ? 'AM' : 'PM'),
      WW: () => pad(this.getWeekNumber(date)),
    };

    // Build a regex that specifically matches the known format keys
    const knownKeys = Object.keys(formatters).map((key) =>
      key.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'),
    ); // Escape regex special chars in keys
    // Sort keys by length descending to match longer keys first (e.g., DDDD before DD)
    knownKeys.sort((a, b) => b.length - a.length);
    const formatRegex = new RegExp(`(${knownKeys.join('|')})`, 'g');

    // Replace only the known keys
    return format.replace(formatRegex, (match) => {
      // The matched key is the first capturing group (the whole match in this case)
      const formatter = formatters[match];
      // If a formatter exists for the key, call it, otherwise return the original match (shouldn't happen with this regex)
      return formatter ? formatter() : match;
    });
  }

  private getWeekNumber(date: Date): number {
    const d = new Date(
      Date.UTC(date.getFullYear(), date.getMonth(), date.getDate()),
    );
    // Set to nearest Thursday: current date + 4 - current day number
    // Make Sunday's day number 7
    d.setUTCDate(d.getUTCDate() + 4 - (d.getUTCDay() || 7));
    // Get first day of year
    const yearStart = new Date(Date.UTC(d.getUTCFullYear(), 0, 1));
    // Calculate full weeks to nearest Thursday
    const weekNo = Math.ceil(
      ((d.getTime() - yearStart.getTime()) / 86400000 + 1) / 7,
    );
    // Return week number
    return weekNo;
  }

  private generateRandomId(): string {
    return crypto.randomBytes(4).toString('hex');
  }
}
