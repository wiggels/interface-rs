# interface-rs Changelog

All notable changes to this project will be documented in this file.


## [0.3.0](https://github.com/wiggels/interface-rs/compare/v0.2.10...0.3.0) - 2026-01-15

- ⛰️ Add strongly typed InterfaceOption enum for interface options ([5e317f1](https://github.com/wiggels/interface-rs/commit/5e317f103dbf4cc32c73736fffb97400da4e4c55))
- 🏗️ Add placeholder changelog ([05228d9](https://github.com/wiggels/interface-rs/commit/05228d9c20118d53a0207b38dc849088ffb70376))
- 🏗️ Add release ([2088ad9](https://github.com/wiggels/interface-rs/commit/2088ad9c0f65ff3229e74f6a14d5e9dddb904f67))
- 🏗️ Replace unwrap with expect and add safety comments in parser ([a2f24d1](https://github.com/wiggels/interface-rs/commit/a2f24d1fd4d8a68540cd31e5bb1f87ff234ed614))
- 🏗️ Add PartialEq derive to Interface and Mapping types ([f78a0d5](https://github.com/wiggels/interface-rs/commit/f78a0d5809b2e322839778622dd46f7ff6e553d5))
- 🏗️ Add get_option and get_options helper methods to Interface ([5c66015](https://github.com/wiggels/interface-rs/commit/5c660152d13d775a91c0016236bcce245e043f3e))
- 🏗️ Remove duplicate FamilyParseError definition from error.rs ([be5f2d7](https://github.com/wiggels/interface-rs/commit/be5f2d7bd24fe441161982981b9d6e5816131443))
- 🏗️ Remove unused MethodParseError from error types ([a341cd4](https://github.com/wiggels/interface-rs/commit/a341cd40c58d4fe2c4950f95a319700c4f1acd5d))
- 🏗️ Change Mapping script field from String to PathBuf ([e5275b0](https://github.com/wiggels/interface-rs/commit/e5275b0ba8a068bfe7a1c8d8d55b3f890f9df0eb))
- 🏗️ Add Method enum to replace String for interface method field ([7df62ff](https://github.com/wiggels/interface-rs/commit/7df62ff593656ee4d749c64a60954b0374d63b6e))
- 🐛 Update README with Method enum examples and fix code samples ([16f9af6](https://github.com/wiggels/interface-rs/commit/16f9af6ff941dd203902f85707728f25410159f9))
- 🐛 Fix non-idiomatic patterns and add Default impl for Parser ([cbb8a6b](https://github.com/wiggels/interface-rs/commit/cbb8a6be52911f1c7a020c1876ed1e35a8b54ab5))
- 📚 Update README to use InterfaceOption typed API ([ccbe9eb](https://github.com/wiggels/interface-rs/commit/ccbe9ebc4f1d6d72960156fe57dc46f48fb09abf))
- 🧪 Add ParserError helper methods and comprehensive error tests ([ddcd839](https://github.com/wiggels/interface-rs/commit/ddcd839e1129463f646e3c4330fe96c143a59102))
- 🧪 Add comprehensive tests for Family, natural sort, and parser ([d67060f](https://github.com/wiggels/interface-rs/commit/d67060fe31f433471721f29be7f4efcf24d1e5dc))

## [0.2.10](https://github.com/wiggels/interface-rs/compare/v0.2.9...v0.2.10) - 2024-11-20

- 🏗️ Release interface-rs version 0.2.10 ([8bad2fd](https://github.com/wiggels/interface-rs/commit/8bad2fde7318998b165253574dbce111160faa06))
- 🏗️ Add get_bridge_interfaces ([6632857](https://github.com/wiggels/interface-rs/commit/66328576af77066c81c9e4165d75287c46590aef))

## [0.2.9](https://github.com/wiggels/interface-rs/compare/v0.2.8...v0.2.9) - 2024-11-20

- 🏗️ Release interface-rs version 0.2.9 ([3b9799a](https://github.com/wiggels/interface-rs/commit/3b9799a59e08442b3344da03a16da5de00f3f2a7))
- 🐛 Fix display ordering logic and simplify save ([c2ffde9](https://github.com/wiggels/interface-rs/commit/c2ffde9ceb1f94151f06252f5f870eff3f0c94e6))

## [0.2.8](https://github.com/wiggels/interface-rs/compare/v0.2.7...v0.2.8) - 2024-11-19

- ↩️ Fix revert flake ([3f35802](https://github.com/wiggels/interface-rs/commit/3f35802be3d66bcdbb5fa588abd37f8c443750cb))
- ↩️ Revert bad sorting addition ([9a1f7e8](https://github.com/wiggels/interface-rs/commit/9a1f7e8ef3d79162511438437972f890857cdf34))
- 🏗️ Release interface-rs version 0.2.8 ([63d2069](https://github.com/wiggels/interface-rs/commit/63d2069c2c35c0faaa1443e89c2102db68699d85))

## [0.2.7](https://github.com/wiggels/interface-rs/compare/v0.2.6...v0.2.7) - 2024-11-19

- 🏗️ Release interface-rs version 0.2.7 ([0a19e61](https://github.com/wiggels/interface-rs/commit/0a19e616b05617c74bd92a31f75d7d62f90b6092))
- 🏗️ Attempt natural sort ([469c063](https://github.com/wiggels/interface-rs/commit/469c063fd32e9eb4e6b945e753fac195b0d62931))

## [0.2.6](https://github.com/wiggels/interface-rs/compare/v0.2.5...v0.2.6) - 2024-11-19

- 🏗️ Release interface-rs version 0.2.6 ([f4fc5d1](https://github.com/wiggels/interface-rs/commit/f4fc5d115668a561d4a6fff45f6ff3858afe272e))
- 🧪 Fix doc tests and add vni/vlan helpers ([46d27ce](https://github.com/wiggels/interface-rs/commit/46d27ce2276b528ebcf1cc87b12572941251bfdc))

## [0.2.5](https://github.com/wiggels/interface-rs/compare/v0.2.4...v0.2.5) - 2024-11-19

- 🏗️ Release interface-rs version 0.2.5 ([b2970e0](https://github.com/wiggels/interface-rs/commit/b2970e04bdffcebbcb64d4e04d3c421191db6b6d))
- 🚜 Make clippy happy and run fmt ([812066f](https://github.com/wiggels/interface-rs/commit/812066f3026424d698320d81a3606e5f9cb0e420))

## [0.2.4](https://github.com/wiggels/interface-rs/compare/v0.2.3...v0.2.4) - 2024-11-19

- 🏗️ Release interface-rs version 0.2.4 ([131f314](https://github.com/wiggels/interface-rs/commit/131f314972980f321fffc1a65a5c0560ce5a65d3))
- 🧪 Add option removal builder functions and tests ([fb20fcb](https://github.com/wiggels/interface-rs/commit/fb20fcb303a436283f0094f4132fe5efd1982b43))

## [0.2.3](https://github.com/wiggels/interface-rs/compare/v0.2.2...v0.2.3) - 2024-11-14

- 🏗️ Release interface-rs version 0.2.3 ([483338e](https://github.com/wiggels/interface-rs/commit/483338e9d74dbd1994165efa0e12732e122344ca))
- 🏗️ Add comment preservation and source support ([534bf5b](https://github.com/wiggels/interface-rs/commit/534bf5b75605d921fccaff667c42da5e912036b6))

## [0.2.2](https://github.com/wiggels/interface-rs/compare/v0.2.1...v0.2.2) - 2024-11-14

- 🏗️ Release interface-rs version 0.2.2 ([141e804](https://github.com/wiggels/interface-rs/commit/141e80438aed03447df1f2b7038c49b0b2abb85b))
- 🐛 Fix display and ordering under certain conditions ([3cfe19e](https://github.com/wiggels/interface-rs/commit/3cfe19ef28985cb51f4109362cfcf3f244db8d88))
- 🧪 Add rust test workflow ([0bf6b17](https://github.com/wiggels/interface-rs/commit/0bf6b1760c7d2089ade113535c1ee05003198add))

## [0.2.1](https://github.com/wiggels/interface-rs/compare/v0.2.0...v0.2.1) - 2024-11-14

- 🧪 Fix parser and add tests ([ab10b35](https://github.com/wiggels/interface-rs/commit/ab10b35564fc7affd653a94f69c9e52a06f1e141))

## [0.2.0](https://github.com/wiggels/interface-rs/releases/tag/v0.2.0) - 2024-11-13

- 🏗️ Create builder pattern ([be386da](https://github.com/wiggels/interface-rs/commit/be386da5520a2ad8d0a72d121397ed085a154698))
- 🏗️ Add readme and bump version ([e0d4e02](https://github.com/wiggels/interface-rs/commit/e0d4e029bf2c0b81aae386fdd653f431d0f4651f))
- 🏗️ Add readme ([4e15d3e](https://github.com/wiggels/interface-rs/commit/4e15d3eecbeae157b782a0dab072f0f8cb8fd925))
- 🏗️ Remove readme ref ([d9767b4](https://github.com/wiggels/interface-rs/commit/d9767b46661bbc56d781675033feeb00fc9d3f2d))
- 🏗️ First real lib version ([0c31cb6](https://github.com/wiggels/interface-rs/commit/0c31cb69f7c4e0e71b701f6dcc651b5142de86a4))
- 🏗️ Init deploy ([63e2b1f](https://github.com/wiggels/interface-rs/commit/63e2b1f6faf7548b5ec7d5268ecd67b7b7d7bc15))
- 🐛 Fix category slug ([effecef](https://github.com/wiggels/interface-rs/commit/effecefb4ff2548bbf409eb838545fbf5c24b29b))
<!-- generated by git-cliff -->
