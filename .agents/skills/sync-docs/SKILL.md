# sync-docs

> Sinkronisasi seluruh dokumen markdown dan artefak.

Periksa dan perbaiki dokumentasi agar konsisten dengan struktur terkini.

## Langkah

### 1. Audit Internal Links

```bash
# Cari semua link .md yang broken di docs/
python3 -c "
import os, re
broken = 0
for root, dirs, files in os.walk('docs/'):
    for f in files:
        if not f.endswith('.md'): continue
        path = os.path.join(root, f)
        with open(path) as fh:
            content = fh.read()
        dir_from = os.path.dirname(path)
        for m in re.finditer(r'\\[([^\\]]*)\\]\\(([^)]*)\\)', content):
            target = m.group(2)
            if not target.endswith('.md') or target.startswith('http'): continue
            resolved = os.path.normpath(os.path.join(dir_from, target))
            if not os.path.exists(resolved):
                broken += 1
                print(f'BROKEN: {os.path.relpath(path)} -> {target}')
print(f'{broken} broken')
"
```

Perbaiki semua broken link.

### 2. Audit Crate References

- Cari referensi ke crate yang sudah tidak ada atau berubah nama
- Cari referensi ke path file yang sudah berubah struktur
- Update jika ditemukan

### 3. Audit URL Eksternal

Cek URL yang mengarah ke `github.com/reasvyn/{old-repo}` dan update ke `reasvyn/rvlibs`.

### 4. Verifikasi Konvensi

Pastikan setiap dokumen di `docs/learn/` mengikuti struktur:

- `## Prerequisites` — prasyarat
- Konten utama
- `## Glossarium` — tabel Term/Definition
- `## Next Steps` — minimal 1 link internal + 1 link eksternal

### 5. Commit

```bash
git add -A
git commit -m "Sync docs: fix broken links and outdated references"
```
