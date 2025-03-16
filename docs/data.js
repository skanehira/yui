window.BENCHMARK_DATA = {
  "lastUpdate": 1742117833916,
  "repoUrl": "https://github.com/skanehira/yui",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "sho19921005@gmail.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "committer": {
            "email": "sho19921005@gmail.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "distinct": true,
          "id": "41a39930a0a5dc15bb34f624f47c4f18fd2d88b6",
          "message": "init",
          "timestamp": "2025-03-15T09:57:47+09:00",
          "tree_id": "4f345d91835a21713efe869cfd6f911185799382",
          "url": "https://github.com/skanehira/yui/commit/41a39930a0a5dc15bb34f624f47c4f18fd2d88b6"
        },
        "date": 1742000418980,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_fib",
            "value": 18.02,
            "range": "± 0.10",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sho19921005@gmail.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "committer": {
            "email": "sho19921005@gmail.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "distinct": true,
          "id": "6b149aba3db4f353ec39a3c5680534dc41321e3c",
          "message": "impl: parse elf header",
          "timestamp": "2025-03-16T16:20:10+09:00",
          "tree_id": "c6c6c5d2d43e4e1059ed64e337b35202b41d942a",
          "url": "https://github.com/skanehira/yui/commit/6b149aba3db4f353ec39a3c5680534dc41321e3c"
        },
        "date": 1742109651421,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_fib",
            "value": 18.02,
            "range": "± 0.44",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sho19921005@gmail.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "committer": {
            "email": "sho19921005@gmail.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "distinct": true,
          "id": "90e66c377d4eb3a85a3174a966dcf11896523c05",
          "message": "chore: refactor something\n\n- add Makefile\n- add bench test\n- simplify test object file",
          "timestamp": "2025-03-16T18:08:27+09:00",
          "tree_id": "85a3339e0f26810381834a95faf3cbc2d3b29443",
          "url": "https://github.com/skanehira/yui/commit/90e66c377d4eb3a85a3174a966dcf11896523c05"
        },
        "date": 1742116152899,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_fib",
            "value": 91.98,
            "range": "± 18.91",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sho19921005@gmail.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "committer": {
            "email": "sho19921005@gmail.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "distinct": true,
          "id": "e32976ad39be18e342802e126e9713410b6a62e6",
          "message": "refactor: ELF parsing and error handling\n\n- Renamed various ELF-related enums and structs for clarity:\n  - `ElfClass` to `Class`\n  - `ElfData` to `Data`\n  - `ElfIdentVersion` to `IdentVersion`\n  - `ElfOSABI` to `OSABI`\n  - `ElfType` to `Type`\n  - `ElfMachine` to `Machine`\n  - `ElfVersion` to `Version`\n  - `ElfHeader` to `Header`\n  - `ElfIdent` to `Ident`\n  - `ElfError` to `ParseError`\n- Updated `parse_elf_header` function to return `Header` instead of `ElfHeader`.\n- Added `parse_elf` function to parse the entire ELF structure.\n- Modified tests to reflect these changes.\n- Updated benchmark function name to `bench_parse_elf_header`.",
          "timestamp": "2025-03-16T18:36:28+09:00",
          "tree_id": "95fb8c5a98ef7eef9e9c32417bc065ddc2bf7fb0",
          "url": "https://github.com/skanehira/yui/commit/e32976ad39be18e342802e126e9713410b6a62e6"
        },
        "date": 1742117833599,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 90.66,
            "range": "± 1.35",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}