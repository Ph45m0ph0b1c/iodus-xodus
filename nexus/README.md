# Nexus patches

The `iodus` branch is Xodus commit `53ea781` plus the 76 backend patches from
[Xodus Nexus](https://github.com/domsvoboda/xodus-nexus) `develop` (`6100e3a`,
MIT license). Each patch is one commit, in the order of `backend-source.lock.json`.

The lock file here is a copy from Nexus, for reference.

## License and changes

The Nexus patches are under the MIT License (`LICENSE-nexus-MIT.txt`,
"Copyright (c) 2026 Xodus Nexus contributors"). The rest of this repository is
Xodus, under the GPL-3.0.

iOdus maintains this code now; the build does not need the Nexus repository.
Each Nexus patch stays one commit, without changes, so that a diff against Nexus
shows only the iOdus changes. The Linux-only code of the patches stays too; the
macOS build does not compile it. The plan is to replace the Nexus code, area by
area, with iOdus code. A replaced area can then drop its Linux-only code.
