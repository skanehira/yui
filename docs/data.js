window.BENCHMARK_DATA = {
  "lastUpdate": 1742220604732,
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
          "id": "c16afd86f402cc955553de356a5c98372de886f0",
          "message": "Refactor ELF module and update benchmarks\n\n- Updated GitHub Actions workflow to not fail on alert.\n- Refactored ELF module by moving enums and structs to `src/elf/header.rs`.\n- Updated benchmarks to use new parser function.\n- Added new files: `src/elf/header.rs` and `src/parser.rs`.\n- Renamed and moved files from `src/elf` to `src/parser`.\n- Fixed tests to accommodate changes in parser functions.",
          "timestamp": "2025-03-16T18:56:40+09:00",
          "tree_id": "4a76ab9c084796ecaec330631d6464a51429cea0",
          "url": "https://github.com/skanehira/yui/commit/c16afd86f402cc955553de356a5c98372de886f0"
        },
        "date": 1742119036278,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 91.18,
            "range": "± 3.13",
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
          "id": "71fea23fdfc3c00ac68daeed51bce44ddb8b8140",
          "message": "Refactor ELF Header Parsing\n\n- Updated field names in `Header` struct for clarity and consistency with ELF specification.\n- Modified `parse_elf` function in `src/parser.rs` to return `IResult` for better error handling.\n- Adjusted `parse` function in `src/parser/header.rs` to align with new `Header` field names.\n- Updated tests in `src/parser/header.rs` to reflect changes in `Header` struct field names.",
          "timestamp": "2025-03-16T19:21:48+09:00",
          "tree_id": "cc4c73b3fb177bdb3c9f7a6935f2f88e92ff2829",
          "url": "https://github.com/skanehira/yui/commit/71fea23fdfc3c00ac68daeed51bce44ddb8b8140"
        },
        "date": 1742120582579,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 91.01,
            "range": "± 3.36",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "07468de1cf821104591f2fcc4c01424a9970a37c",
          "message": "Merge pull request #1 from skanehira/impl-section-parse\n\nimpl section parse",
          "timestamp": "2025-03-16T22:53:41+09:00",
          "tree_id": "0ec195c73f1c23bb05c848103b880150fff7055e",
          "url": "https://github.com/skanehira/yui/commit/07468de1cf821104591f2fcc4c01424a9970a37c"
        },
        "date": 1742133257012,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 98.8,
            "range": "± 9.68",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 871.9,
            "range": "± 18.53",
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
          "id": "fffce61f6571d2a664bdb6365d2d86fb4201fd87",
          "message": "fix: remove unused module and update variable value",
          "timestamp": "2025-03-17T23:09:16+09:00",
          "tree_id": "a5f6a14d10dfd0b24cebefde88657dec8113b321",
          "url": "https://github.com/skanehira/yui/commit/fffce61f6571d2a664bdb6365d2d86fb4201fd87"
        },
        "date": 1742220604459,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 91.9,
            "range": "± 2.01",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 849.37,
            "range": "± 211.12",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}