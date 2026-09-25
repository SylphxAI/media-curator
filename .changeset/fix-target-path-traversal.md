---
'@sylphx/media-curator': patch
---

Security: a file's EXIF camera model or name can no longer write outside the
target, duplicate, or error directory. Each placeholder value fills at most one
sanitised path segment, and every destination is checked to stay inside its
directory. Also fixes: files with no duplicate are transferred again, cached
hashes inside objects survive the LMDB cache, cached metadata reads no longer
fail, and discovery accepts individual files as sources.
