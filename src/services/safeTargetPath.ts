import { isAbsolute, relative, resolve, sep } from 'path';

// Path separators, Windows-reserved characters, and ASCII control characters.
// eslint-disable-next-line no-control-regex
const UNSAFE_SEGMENT_CHARS = /[/\\<>:"|?*\u0000-\u001f\u007f]/g;

/**
 * Turn an untrusted value (EXIF camera model, file name, format literal) into
 * a single path segment: no separators, no control characters, and never `.`
 * or `..`. Returns `_` when nothing safe remains.
 */
export function sanitizePathSegment(value: string): string {
  const cleaned = value.replace(UNSAFE_SEGMENT_CHARS, '_').trim();
  if (cleaned === '' || /^\.+$/.test(cleaned)) {
    return '_';
  }
  return cleaned;
}

/**
 * Resolve `segments` under `root` and fail closed if the result escapes it.
 * Callers sanitise each segment first; this is the final containment check.
 */
export function resolveInside(root: string, ...segments: string[]): string {
  const base = resolve(root);
  const target = resolve(base, ...segments);
  const rel = relative(base, target);
  if (
    rel === '' ||
    rel === '..' ||
    rel.startsWith(`..${sep}`) ||
    isAbsolute(rel)
  ) {
    throw new Error(
      `Refusing to write outside the target directory: ${target} is not inside ${base}`,
    );
  }
  return target;
}
