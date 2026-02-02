window.BENCHMARK_DATA = {
  "lastUpdate": 1769993100632,
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
          "id": "9246b6c699f7fcba36d75249b6149e52e700f38d",
          "message": "chore: add Makefile for build test fixitures",
          "timestamp": "2025-03-23T21:39:36+09:00",
          "tree_id": "93016853b019781da9cd255d7adc818121c00357",
          "url": "https://github.com/skanehira/yui/commit/9246b6c699f7fcba36d75249b6149e52e700f38d"
        },
        "date": 1742733618493,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 92.2,
            "range": "± 34.09",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 854.11,
            "range": "± 357.76",
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
          "id": "482b19dafd3739e45bb3be320a48951d7b2859a1",
          "message": "chore: if section flag is undefined, make flags field empty",
          "timestamp": "2025-03-23T23:21:38+09:00",
          "tree_id": "f87ab715b2a6dbb132bdcb4f549becc1c385dd87",
          "url": "https://github.com/skanehira/yui/commit/482b19dafd3739e45bb3be320a48951d7b2859a1"
        },
        "date": 1742739739519,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 97.45,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 787.96,
            "range": "± 431.79",
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
          "id": "43d12f1eae5a376f4a025527edc13eb12b788f96",
          "message": "Add relocation parsing to ELF parser\n\n- Introduced `relocation` module in `src/elf.rs` and `src/parser.rs`.\n- Added `RelocationType`, `Info`, and `RelocationAddend` structs in `src/elf/relocation.rs`.\n- Implemented parsing logic for relocation addends in `src/parser/relocation.rs`.\n- Updated `Elf` struct to include optional relocation data.\n- Enhanced error handling with `InvalidRelocationType` in `src/parser/error.rs`.\n- Added tests for relocation parsing functionality.",
          "timestamp": "2025-03-24T23:44:42+09:00",
          "tree_id": "92014627494b0dd92011633dcc22a1d873524e3a",
          "url": "https://github.com/skanehira/yui/commit/43d12f1eae5a376f4a025527edc13eb12b788f96"
        },
        "date": 1742827521416,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 90.54,
            "range": "± 4.10",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 821.75,
            "range": "± 14.37",
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
          "id": "79871e95a3d8f3b02573d5ff57601a3ab9a43194",
          "message": "Refactor ELF parsing: Change `relocation` to `relocations` for consistency\n\n- Updated `Elf` struct to use `Vec<relocation::RelocationAddend>` instead of `Option<Vec<relocation::RelocationAddend>>`.\n- Modified `parse_elf` function to handle the updated `relocations` field.\n- Adjusted `parse` function in `relocation.rs` to return an empty vector instead of `None` when no relocations are found.\n- Updated tests to reflect changes in the `relocations` handling.",
          "timestamp": "2025-03-25T21:42:36+09:00",
          "tree_id": "660726a8e072c2ade300e72841fae09b24947647",
          "url": "https://github.com/skanehira/yui/commit/79871e95a3d8f3b02573d5ff57601a3ab9a43194"
        },
        "date": 1742906596525,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 90.96,
            "range": "± 1.85",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 811.61,
            "range": "± 102.25",
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
          "id": "e3b53fd61f85f1958926074b51b0d000616af193",
          "message": "Fix ELF parsing logic and handle empty section headers\n\n- Removed unnecessary check for zero section headers in `parse_elf` function.\n- Simplified section header parsing by directly calling `section::parse_header`.\n- Corrected variable name from `relocation` to `relocations`.\n- Added handling for zero section headers in `parse_header` function in `section.rs`.",
          "timestamp": "2025-03-26T07:54:30+09:00",
          "tree_id": "a615f7f5ee6cccb4785a31f07bca04cf19d8fd78",
          "url": "https://github.com/skanehira/yui/commit/e3b53fd61f85f1958926074b51b0d000616af193"
        },
        "date": 1742943312363,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 91.28,
            "range": "± 3.29",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 800.85,
            "range": "± 15.08",
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
          "id": "a61cf3be08a78c97ca7bede4ee35dee8b4ff7217",
          "message": "Add support for parsing section header table data in ELF files\n\n- Updated `Elf` and `Header` structs to include section data.\n- Modified `parse_header` function to extract and store section data.\n- Integrated `insta` for snapshot testing.\n- Replaced manual assertions with snapshot testing in `should_parse_section_header_table` test.",
          "timestamp": "2025-03-26T16:14:10+09:00",
          "tree_id": "e20018e4216fc93227ee33044b3afe37328f45c5",
          "url": "https://github.com/skanehira/yui/commit/a61cf3be08a78c97ca7bede4ee35dee8b4ff7217"
        },
        "date": 1742973294044,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 91.7,
            "range": "± 2.02",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 889,
            "range": "± 20.12",
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
          "id": "02f9e9e64caf255be2f44a2451c302ca63a0064a",
          "message": "Merge pull request #3 from skanehira/impl-linker\n\nfeat: Add ELF linker implementation",
          "timestamp": "2025-04-27T15:37:50+09:00",
          "tree_id": "431775867bee72bed29568aadf070f4eb2750026",
          "url": "https://github.com/skanehira/yui/commit/02f9e9e64caf255be2f44a2451c302ca63a0064a"
        },
        "date": 1745735910895,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 92.47,
            "range": "± 2.73",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1139.22,
            "range": "± 10.94",
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
          "id": "e7f783ffe5f7c9f7625f7bd7178273b3f077cce2",
          "message": "feat(linker): translate error messages to English\n\nTranslated all Japanese error messages in `linker.rs` to English for\nbetter readability and consistency across the codebase. This improves\nmaintainability and accessibility for non-Japanese-speaking developers.",
          "timestamp": "2025-04-28T00:07:52+09:00",
          "tree_id": "f2fe27076b7d2627366223bcf471caf34e10860e",
          "url": "https://github.com/skanehira/yui/commit/e7f783ffe5f7c9f7625f7bd7178273b3f077cce2"
        },
        "date": 1745766547666,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 94.09,
            "range": "± 1.27",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1126.35,
            "range": "± 16.13",
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
          "id": "3b0c5c619188158150b48f3075c90466a8438668",
          "message": "Merge pull request #4 from skanehira/dependabot/cargo/insta-1.43.0\n\nchore(deps): bump insta from 1.42.2 to 1.43.0",
          "timestamp": "2025-04-28T09:56:32+09:00",
          "tree_id": "521d5eed015f89b9b49282e85ef03aeefcfa9aa8",
          "url": "https://github.com/skanehira/yui/commit/3b0c5c619188158150b48f3075c90466a8438668"
        },
        "date": 1745801835766,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 93.92,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1072.09,
            "range": "± 14.14",
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
          "id": "49ddee2f3ec6bb479d00a7d6001dcbf5ccdc05da",
          "message": "feat(linker): refactor and document section merging logic\n\nRefactored the `merge_sections` method to consolidate `.text` and `.data`\nsections from multiple ELF objects into a single output executable.\nRemoved redundant methods `has_symbol` and `has_undefined_symbol` to\nstreamline the codebase.\n\nAdded detailed documentation comments to the `merge_sections` method,\nexplaining its purpose, arguments, and return values.\n\nUpdated tests to reflect changes in section merging and address\ncalculations.",
          "timestamp": "2025-04-29T13:29:06+09:00",
          "tree_id": "e4a736f357df76ed983cffe444c1a53dba0c08e9",
          "url": "https://github.com/skanehira/yui/commit/49ddee2f3ec6bb479d00a7d6001dcbf5ccdc05da"
        },
        "date": 1745901013793,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 93.5,
            "range": "± 2.88",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1051.51,
            "range": "± 10.15",
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
          "id": "3def8dd52e08d1f7beefd1d3bc4150448fb4e683",
          "message": "feat(linker): improve section alignment and base address handling\n\nIntroduce a static `BASE_ADDR` for consistent base address usage. Refactor\nsection alignment and offsets to improve ELF section placement logic.\nUpdate `make_symbol_section` to accept the latest section address for\ndynamic offset calculation.\n\nBREAKING CHANGE: Adjustments to section alignment and offsets may affect\nexisting ELF outputs.",
          "timestamp": "2025-04-30T01:44:25+09:00",
          "tree_id": "aa02f414f528a92d067f89eacf4d833ab9defaa6",
          "url": "https://github.com/skanehira/yui/commit/3def8dd52e08d1f7beefd1d3bc4150448fb4e683"
        },
        "date": 1745945108624,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 93.29,
            "range": "± 1.48",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1010.84,
            "range": "± 11.99",
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
          "id": "8c7ee2c36aaac1df35560c4fcec3d73df7024a86",
          "message": "chore: fmt",
          "timestamp": "2025-05-03T01:57:01+09:00",
          "tree_id": "481a3e6cc0c8d9baebfd64b96f11d9daefcd0d99",
          "url": "https://github.com/skanehira/yui/commit/8c7ee2c36aaac1df35560c4fcec3d73df7024a86"
        },
        "date": 1746205075437,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 101.77,
            "range": "± 6.46",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1020.49,
            "range": "± 15.95",
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
          "id": "f32c7fb64ffaee5421aabaac4e771b1ec9b68311",
          "message": "ci: add test (#5)\n\n* ci: add test\n\n* fix: ci fail\n\n* debug\n\n* ci: add trigger for tests\n\n* fix: ci fail",
          "timestamp": "2025-05-03T07:58:19+09:00",
          "tree_id": "a4d84d942b11c1c68ed9cd02a7e193106156af30",
          "url": "https://github.com/skanehira/yui/commit/f32c7fb64ffaee5421aabaac4e771b1ec9b68311"
        },
        "date": 1746226737929,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 91.56,
            "range": "± 1.72",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1033.81,
            "range": "± 9.91",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1698246662252ba98ad08a7422e4edb28c37f661",
          "message": "chore(deps): bump insta from 1.43.0 to 1.43.1 (#6)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.43.0 to 1.43.1.\n- [Release notes](https://github.com/mitsuhiko/insta/releases)\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.43.0...1.43.1)\n\n---\nupdated-dependencies:\n- dependency-name: insta\n  dependency-version: 1.43.1\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-05-05T10:54:10+09:00",
          "tree_id": "8675d2a6894e884b3757f1476a5631af8aab4db0",
          "url": "https://github.com/skanehira/yui/commit/1698246662252ba98ad08a7422e4edb28c37f661"
        },
        "date": 1746410090048,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 92.57,
            "range": "± 2.24",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1007.95,
            "range": "± 14.61",
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
          "id": "630fd20f6965c3b0af5429bcf51a6a0298b3bfd5",
          "message": "fix(parser): correct string table slicing logic\n\nEnsure proper slicing of string tables in `section.rs` and `symbol.rs`\nby including both offset and size. This resolves potential out-of-bound\nerrors when accessing string tables.\n\nAffected files:\n- `src/parser/section.rs`\n- `src/parser/symbol.rs`",
          "timestamp": "2025-05-11T22:39:05+09:00",
          "tree_id": "30a8a184a16ad04ee88ccd8d82d28cebb1099236",
          "url": "https://github.com/skanehira/yui/commit/630fd20f6965c3b0af5429bcf51a6a0298b3bfd5"
        },
        "date": 1746977419231,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 90.98,
            "range": "± 1.46",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1028.12,
            "range": "± 14.77",
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
          "id": "4605c5ae07f1e6cd0d802470c2776d7f36af1348",
          "message": "chore: remove unused code",
          "timestamp": "2025-05-13T21:50:49+09:00",
          "tree_id": "2fdd4158c36d2e7379330075b544e761afbfde65",
          "url": "https://github.com/skanehira/yui/commit/4605c5ae07f1e6cd0d802470c2776d7f36af1348"
        },
        "date": 1747140698249,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 91.64,
            "range": "± 1.95",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1051.18,
            "range": "± 33.63",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "251d486cceb6ed345f51b482d08d2d1861a37859",
          "message": "chore(deps): bump actions/checkout from 4 to 5 (#9)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 4 to 5.\n- [Release notes](https://github.com/actions/checkout/releases)\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/actions/checkout/compare/v4...v5)\n\n---\nupdated-dependencies:\n- dependency-name: actions/checkout\n  dependency-version: '5'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-08-18T14:33:15+09:00",
          "tree_id": "45f9f15bebe4778b1e034f71313e521659db7205",
          "url": "https://github.com/skanehira/yui/commit/251d486cceb6ed345f51b482d08d2d1861a37859"
        },
        "date": 1755495224209,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 92.4,
            "range": "± 2.43",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1023.13,
            "range": "± 19.84",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "90f4242b01f7058ea45e65a21c9f9974fb0831f9",
          "message": "chore(deps): bump insta from 1.43.1 to 1.43.2 (#10)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.43.1 to 1.43.2.\n- [Release notes](https://github.com/mitsuhiko/insta/releases)\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.43.1...1.43.2)\n\n---\nupdated-dependencies:\n- dependency-name: insta\n  dependency-version: 1.43.2\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-09-08T08:54:37+09:00",
          "tree_id": "f03e4361e8a2b73e5b1fe39e3e6b0ca8b7a8d5e8",
          "url": "https://github.com/skanehira/yui/commit/90f4242b01f7058ea45e65a21c9f9974fb0831f9"
        },
        "date": 1757289305201,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 93.83,
            "range": "± 2.62",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 989.18,
            "range": "± 9.53",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "02081fdaa2cc1189d104a7673f5158ff8d3d1bed",
          "message": "chore(deps): bump actions/checkout from 5 to 6 (#11)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 5 to 6.\n- [Release notes](https://github.com/actions/checkout/releases)\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/actions/checkout/compare/v5...v6)\n\n---\nupdated-dependencies:\n- dependency-name: actions/checkout\n  dependency-version: '6'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-11-24T12:52:34+09:00",
          "tree_id": "39cd4c9576743e651ab4f0ff534e59454998f7d5",
          "url": "https://github.com/skanehira/yui/commit/02081fdaa2cc1189d104a7673f5158ff8d3d1bed"
        },
        "date": 1763956381937,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 85.84,
            "range": "± 1.46",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 994.25,
            "range": "± 16.08",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "066ab137a36ea4fb924614666f0d1cd7ffa1ab95",
          "message": "chore(deps): bump insta from 1.43.2 to 1.44.1 (#12)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.43.2 to 1.44.1.\n- [Release notes](https://github.com/mitsuhiko/insta/releases)\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.43.2...1.44.1)\n\n---\nupdated-dependencies:\n- dependency-name: insta\n  dependency-version: 1.44.1\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-11-24T12:53:46+09:00",
          "tree_id": "b3e0456d0c3989b834b1461c5d1aa1b7c3a63d93",
          "url": "https://github.com/skanehira/yui/commit/066ab137a36ea4fb924614666f0d1cd7ffa1ab95"
        },
        "date": 1763956451609,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 85.87,
            "range": "± 2.24",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 984.3,
            "range": "± 11.6",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1de8c50f18f63db83f96869b0846fa1ef51f6273",
          "message": "chore(deps): bump insta from 1.44.1 to 1.45.0 (#14)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.44.1 to 1.45.0.\n- [Release notes](https://github.com/mitsuhiko/insta/releases)\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.44.1...1.45.0)\n\n---\nupdated-dependencies:\n- dependency-name: insta\n  dependency-version: 1.45.0\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-12-22T10:18:24+09:00",
          "tree_id": "4b631ee1ba35065cd336fe75cdb77e30efc86f97",
          "url": "https://github.com/skanehira/yui/commit/1de8c50f18f63db83f96869b0846fa1ef51f6273"
        },
        "date": 1766366332555,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 93.5,
            "range": "± 3.05",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 969.06,
            "range": "± 11.5",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kanehira.sho@techlead-it.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "committer": {
            "email": "kanehira.sho@techlead-it.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "distinct": true,
          "id": "a2a6e95c3a3114c382628c5829551f7ec1a1a052",
          "message": "fix: update doctest path for align function\n\nUse public re-export path yui::linker::align instead of private\nmodule path yui::linker::section::align.\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n\nCo-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>",
          "timestamp": "2026-01-02T12:00:10+09:00",
          "tree_id": "a75777c436ba201d589f0cdd187c166812105c94",
          "url": "https://github.com/skanehira/yui/commit/a2a6e95c3a3114c382628c5829551f7ec1a1a052"
        },
        "date": 1767322851786,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 95.63,
            "range": "± 1.32",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 965.91,
            "range": "± 8.41",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kanehira.sho@techlead-it.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "committer": {
            "email": "kanehira.sho@techlead-it.com",
            "name": "skanehira",
            "username": "skanehira"
          },
          "distinct": true,
          "id": "86a27ff3a39e8487a7176bca4c08cb4af873667f",
          "message": "chore: fix fmt",
          "timestamp": "2026-01-02T12:04:28+09:00",
          "tree_id": "4727eff0e4dd232d3d0e07cec5d1b8d1f42e820f",
          "url": "https://github.com/skanehira/yui/commit/86a27ff3a39e8487a7176bca4c08cb4af873667f"
        },
        "date": 1767323099412,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 95.65,
            "range": "± 1.6",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 988.57,
            "range": "± 19.88",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ed7407e8a3c1fe815547e5205076e3aa2fd3397a",
          "message": "chore(deps): bump insta from 1.45.0 to 1.46.0 (#15)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.45.0 to 1.46.0.\n- [Release notes](https://github.com/mitsuhiko/insta/releases)\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.45.0...1.46.0)\n\n---\nupdated-dependencies:\n- dependency-name: insta\n  dependency-version: 1.46.0\n  dependency-type: direct:production\n  update-type: version-update:semver-minor\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-01-05T14:14:18+09:00",
          "tree_id": "42f6949a431c221fcc45d548519b7dfd6de546ba",
          "url": "https://github.com/skanehira/yui/commit/ed7407e8a3c1fe815547e5205076e3aa2fd3397a"
        },
        "date": 1767590087718,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 94.16,
            "range": "± 1.72",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1003.17,
            "range": "± 15.25",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "84588c6b46d1bea9a7361ba424955a41f05aafd3",
          "message": "chore(deps): bump thiserror from 2.0.17 to 2.0.18 (#16)\n\nBumps [thiserror](https://github.com/dtolnay/thiserror) from 2.0.17 to 2.0.18.\n- [Release notes](https://github.com/dtolnay/thiserror/releases)\n- [Commits](https://github.com/dtolnay/thiserror/compare/2.0.17...2.0.18)\n\n---\nupdated-dependencies:\n- dependency-name: thiserror\n  dependency-version: 2.0.18\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-01-19T10:10:03+09:00",
          "tree_id": "f270d6fc57627640743710fbe081b2530eda4c36",
          "url": "https://github.com/skanehira/yui/commit/84588c6b46d1bea9a7361ba424955a41f05aafd3"
        },
        "date": 1768785037213,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 95.55,
            "range": "± 3.65",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1023.06,
            "range": "± 42.18",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "90690733f72bc9a0f13bc2fff66f2afaa58c0686",
          "message": "chore(deps): bump insta from 1.46.0 to 1.46.1 (#17)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.46.0 to 1.46.1.\n- [Release notes](https://github.com/mitsuhiko/insta/releases)\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.46.0...1.46.1)\n\n---\nupdated-dependencies:\n- dependency-name: insta\n  dependency-version: 1.46.1\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-01-19T10:10:12+09:00",
          "tree_id": "62dc624f5f74a785dd6c52b6820ede12d35f96ce",
          "url": "https://github.com/skanehira/yui/commit/90690733f72bc9a0f13bc2fff66f2afaa58c0686"
        },
        "date": 1768785043235,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 95.33,
            "range": "± 3.6",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 1037.48,
            "range": "± 48.89",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "53aaeb5eb0baf48f8630ed423288516ac21ecd29",
          "message": "chore(deps): bump insta from 1.46.1 to 1.46.2 (#18)\n\nBumps [insta](https://github.com/mitsuhiko/insta) from 1.46.1 to 1.46.2.\n- [Release notes](https://github.com/mitsuhiko/insta/releases)\n- [Changelog](https://github.com/mitsuhiko/insta/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/mitsuhiko/insta/compare/1.46.1...1.46.2)\n\n---\nupdated-dependencies:\n- dependency-name: insta\n  dependency-version: 1.46.2\n  dependency-type: direct:production\n  update-type: version-update:semver-patch\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-02-02T09:44:29+09:00",
          "tree_id": "5901beac5d7493a31db53aac12c2ee5b99bf3445",
          "url": "https://github.com/skanehira/yui/commit/53aaeb5eb0baf48f8630ed423288516ac21ecd29"
        },
        "date": 1769993099851,
        "tool": "cargo",
        "benches": [
          {
            "name": "tests::bench_parse_elf_header",
            "value": 71.68,
            "range": "± 0.46",
            "unit": "ns/iter"
          },
          {
            "name": "tests::bench_parse_section_header_table",
            "value": 874.51,
            "range": "± 9.47",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}