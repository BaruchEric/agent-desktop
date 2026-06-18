# Changelog

## Unreleased

### Notes

* **ffi:** `AdRefEntry` carries additional reliability metadata in this branch; C ABI consumers must rebuild against the updated `agent_desktop.h` header.

## [0.2.4](https://github.com/BaruchEric/agent-desktop/compare/v0.2.3...v0.2.4) (2026-06-18)


### Features

* add app-tree and menu-path adapter trait methods ([12c361d](https://github.com/BaruchEric/agent-desktop/commit/12c361d5dfdd3358a03af3087262ba40dc29b48b))
* add appearance system-control command ([16bd86d](https://github.com/BaruchEric/agent-desktop/commit/16bd86d0c1284591df6a203621a4484e87ea535f))
* add applescript/jxa/open-url/open-path escape-hatch commands ([bc05106](https://github.com/BaruchEric/agent-desktop/commit/bc05106d47d978efcbede9bf6bbab854cb9a3625))
* add extras-menubar and dock snapshot surfaces ([701c802](https://github.com/BaruchEric/agent-desktop/commit/701c802109dd5a399ed9355f263981e7ce49c4d2))
* add gated run-shell escape hatch with audit log ([ac66bc9](https://github.com/BaruchEric/agent-desktop/commit/ac66bc9ada1bc4a54e2a1c294cd0ff35fbf8cd69))
* add macOS menu title-path walk with AXPress activation ([47600ea](https://github.com/BaruchEric/agent-desktop/commit/47600eadb3600a4042a4df25b77a8ab632045a0d))
* add menu command for title-path activation ([d47efac](https://github.com/BaruchEric/agent-desktop/commit/d47efaccb7f03fcdefb9e8a98618c2a77737bc3b))
* add session-scoped reliability diagnostics ([53b2ec4](https://github.com/BaruchEric/agent-desktop/commit/53b2ec4e355ac037c34016e2a9ef1f01d730e988))
* add strict ref reliability core ([c25f8b3](https://github.com/BaruchEric/agent-desktop/commit/c25f8b3b66e35235b71c5fe28c55f6892fff8745))
* add system-control domain types ([71d9487](https://github.com/BaruchEric/agent-desktop/commit/71d94876c619cde79fd7f5f70f326ade68559eab))
* add SystemController trait and adapter accessor ([ebeefc5](https://github.com/BaruchEric/agent-desktop/commit/ebeefc5f5be2d034cc1059ac15e5647a15557fb8))
* add volume system-control command ([1657643](https://github.com/BaruchEric/agent-desktop/commit/16576433356e2ae26a4e83c87f521329df34aae1))
* add wifi system-control command ([9a453c6](https://github.com/BaruchEric/agent-desktop/commit/9a453c6fb627ed3b4ba9b32e45f9e12bbea7cda8))
* implement macOS appearance control via osascript ([5dba741](https://github.com/BaruchEric/agent-desktop/commit/5dba7419658d1fe8d680dc81fa79da43bbcbfcb2))
* implement macOS CoreAudio volume control ([ecc7c63](https://github.com/BaruchEric/agent-desktop/commit/ecc7c63b33a077649861669945411af326efd7a6))
* implement macOS external execution with timeout and capture ([0e074d8](https://github.com/BaruchEric/agent-desktop/commit/0e074d8bc53e571d4921f750e5c24dbab454422d))
* implement macOS wifi power control via networksetup ([ac55acf](https://github.com/BaruchEric/agent-desktop/commit/ac55acf661ffa48609f518a70f4dc5cbce8797b4))
* snapshot windowless apps via app-root tree ([59f6d62](https://github.com/BaruchEric/agent-desktop/commit/59f6d62ab81948b23972325d7b78743aa7cf0a4e))


### Bug Fixes

* address reliability follow-ups ([2b1cf07](https://github.com/BaruchEric/agent-desktop/commit/2b1cf07561e2fb13005e6c985559b5a516d81e5a))
* close final reliability edge cases ([76c5075](https://github.com/BaruchEric/agent-desktop/commit/76c50756a3bb22a4e555082a989d09be92c5009f))
* close reliability review findings ([7bcf9ab](https://github.com/BaruchEric/agent-desktop/commit/7bcf9ab38db26d5fa127c4eaf78294dfb1fc2aea))
* drain external-exec output via run_with_timeout to prevent pipe deadlock ([bfd662e](https://github.com/BaruchEric/agent-desktop/commit/bfd662eb3a53ea327310689aff929ffc236131e3))
* fail closed on uncertain ref fallback ([f037a84](https://github.com/BaruchEric/agent-desktop/commit/f037a8460c9c8ad75e0012f52c48dc47665c0402))
* harden ref action reliability ([5ba88d5](https://github.com/BaruchEric/agent-desktop/commit/5ba88d5af35dac7afceac4e8ce0396125a89ca60))
* harden ref action reliability ([2f0dda9](https://github.com/BaruchEric/agent-desktop/commit/2f0dda920eccb94c9c70c2f7534d040c4c916c42))
* harden ref action reliability ([81685ba](https://github.com/BaruchEric/agent-desktop/commit/81685babdd07901e9f8b7a410fa1d124c163c659))
* harden ref fallback resolution ([57cc87b](https://github.com/BaruchEric/agent-desktop/commit/57cc87b2604cc8fb537d9418cce0f43c35b55086))
* harden ref reliability edge cases ([5b1c4b9](https://github.com/BaruchEric/agent-desktop/commit/5b1c4b98e6906964af431d348e619524355553a4))
* harden reliability edge cases ([19b5c6a](https://github.com/BaruchEric/agent-desktop/commit/19b5c6a54b1fd0ff027a422f7edef5efa3dd1c79))
* harden source-window ref resolution ([872e395](https://github.com/BaruchEric/agent-desktop/commit/872e395e519d27170617a8780d1f1a1e6d6518a0))
* harden wait and ref action reliability ([86bcef2](https://github.com/BaruchEric/agent-desktop/commit/86bcef27b2b7b15d1ded8006499d1966ea9eb090))
* make explicit snapshots session-independent ([0fc858c](https://github.com/BaruchEric/agent-desktop/commit/0fc858c3662be209dc4d20088765b5ddcbb818ac))
* preserve safe window title fallback ([eeb4303](https://github.com/BaruchEric/agent-desktop/commit/eeb4303e74e3c9fb405c58026eb045c29e086ee9))
* register menu command in batch, policy, and coverage contracts ([4835409](https://github.com/BaruchEric/agent-desktop/commit/48354094f0902aee5caf28e37fbdfa0d6dd57832))
* report wait and trace failure context ([bd75878](https://github.com/BaruchEric/agent-desktop/commit/bd758788cd38879a8685c6c6773eb323e11127f4))
* resolve menu and dock item refs by identity, not bounds ([6761a5d](https://github.com/BaruchEric/agent-desktop/commit/6761a5dc01f852aa505998bd02b89d75717a1004))
* resolve refs by CGWindowID bridge instead of AXWindowNumber ([320b078](https://github.com/BaruchEric/agent-desktop/commit/320b078d96d455adec87ab64260ab9f9ed3d6675))
* scope-verify menu-bar and dock surface roots for ref resolution ([a215a92](https://github.com/BaruchEric/agent-desktop/commit/a215a92b7e53ccbbff334e4576662b00b89a2d60))
* skip finder pseudo windows for snapshots ([4fc097b](https://github.com/BaruchEric/agent-desktop/commit/4fc097b9948bf481421039bb85e9de9669101a3a))
* stabilize macos ref resolution ([a4c8c55](https://github.com/BaruchEric/agent-desktop/commit/a4c8c55b47f5688d0639a6183f62fb16494e1636))

## [0.2.3](https://github.com/lahfir/agent-desktop/compare/v0.2.2...v0.2.3) (2026-06-06)


### Bug Fixes

* harden macos ax window fallback ([3b266fd](https://github.com/lahfir/agent-desktop/commit/3b266fdf040bf83438f69a400b032fd12b8715c6))
* resolve fullscreen AX tree retrieval returning ref_count: 0 ([a52b7c7](https://github.com/lahfir/agent-desktop/commit/a52b7c704a4d13fdc0d6b72f4080bb0fc118be64))

## [0.2.2](https://github.com/lahfir/agent-desktop/compare/v0.2.1...v0.2.2) (2026-06-02)


### Bug Fixes

* **macos:** guard CFArray casts with type-ID check (fixes Mail.app crash) ([#50](https://github.com/lahfir/agent-desktop/issues/50)) ([c02cb5e](https://github.com/lahfir/agent-desktop/commit/c02cb5ecb7314f053d63937437e5f5ba48de3209))

## [0.2.1](https://github.com/lahfir/agent-desktop/compare/v0.2.0...v0.2.1) (2026-05-23)


### Bug Fixes

* stabilize empty accessibility identity refs ([1fb5a7d](https://github.com/lahfir/agent-desktop/commit/1fb5a7d51eb798100b4d597c755fee1161e298bf))

## [0.2.0](https://github.com/lahfir/agent-desktop/compare/v0.1.14...v0.2.0) (2026-05-20)


### ⚠ BREAKING CHANGES

* chain execution deadlines now return TIMEOUT instead of ACTION_FAILED when the target app does not respond before the chain deadline.

### Refactoring

* unify command execution contracts ([1291a9c](https://github.com/lahfir/agent-desktop/commit/1291a9cdbf0566424d38da1eab397d6d4091c06c))

## [0.1.14](https://github.com/lahfir/agent-desktop/compare/v0.1.13...v0.1.14) (2026-05-04)


### Features

* bundle skill docs and refactor --help for AI agents ([#36](https://github.com/lahfir/agent-desktop/issues/36)) ([b04d6f9](https://github.com/lahfir/agent-desktop/commit/b04d6f97317af67648890d2d3b5ead0d27c466c9))

## [0.1.13](https://github.com/lahfir/agent-desktop/compare/v0.1.12...v0.1.13) (2026-04-17)


### Features

* **ffi:** ship C-ABI cdylib with review fixes and release pipeline ([#26](https://github.com/lahfir/agent-desktop/issues/26)) ([3cffbd6](https://github.com/lahfir/agent-desktop/commit/3cffbd67f6b27f42001643bef9fd2530cb7f9003))

## [0.1.12](https://github.com/lahfir/agent-desktop/compare/v0.1.11...v0.1.12) (2026-04-16)


### Features

* progressive skeleton traversal with ref-rooted drill-down ([#20](https://github.com/lahfir/agent-desktop/issues/20)) ([c17f2fa](https://github.com/lahfir/agent-desktop/commit/c17f2fae7abbbe2c914a050fa9e9be5fca9c6af0))

## [0.1.11](https://github.com/lahfir/agent-desktop/compare/v0.1.10...v0.1.11) (2026-03-03)


### Bug Fixes

* show skill install prompt on all success paths ([39b2bc6](https://github.com/lahfir/agent-desktop/commit/39b2bc63480890f7ed417b2c040eecf80c4628a0))

## [0.1.10](https://github.com/lahfir/agent-desktop/compare/v0.1.9...v0.1.10) (2026-03-03)


### Bug Fixes

* add clawhub login step before sync in CI ([208af12](https://github.com/lahfir/agent-desktop/commit/208af12459fea2255e1c80b8cdc9ac420316d769))

## [0.1.9](https://github.com/lahfir/agent-desktop/compare/v0.1.8...v0.1.9) (2026-03-03)


### Features

* scalable skill architecture with ClawHub auto-publishing ([#14](https://github.com/lahfir/agent-desktop/issues/14)) ([9766520](https://github.com/lahfir/agent-desktop/commit/97665203a464e605bc9b156ec90029c5909399be))

## [0.1.8](https://github.com/lahfir/agent-desktop/compare/v0.1.7...v0.1.8) (2026-03-01)


### Features

* add electron/web app compatibility for accessibility tree traversal ([a19c1b5](https://github.com/lahfir/agent-desktop/commit/a19c1b5132d3b71c5de58886ba51357ffc9bd1e8))
* implement --compact flag to collapse single-child unnamed nodes ([4a300c8](https://github.com/lahfir/agent-desktop/commit/4a300c8cb054462ca95fb5160e89e8fce661ec3b))

## [0.1.7](https://github.com/lahfir/agent-desktop/compare/v0.1.6...v0.1.7) (2026-02-28)


### Features

* add notification command types, adapter trait, and CLI wiring ([c5b05ba](https://github.com/lahfir/agent-desktop/commit/c5b05bab600aafa36c642f21837c44583b36459c))
* add notification management commands (macOS) ([b1fd368](https://github.com/lahfir/agent-desktop/commit/b1fd368f195640642adf011b75cca6ecb9e5acc3))
* **macos:** add NC session RAII guard and notification adapter wiring ([0d55c21](https://github.com/lahfir/agent-desktop/commit/0d55c21de0f0ea6ea08ccb836e5527c9da513620))
* **macos:** implement dismiss and notification action commands ([53d697d](https://github.com/lahfir/agent-desktop/commit/53d697d52248c3fa06797787b1eb549ac2766533))
* **macos:** implement notification list via AX tree traversal ([53549a3](https://github.com/lahfir/agent-desktop/commit/53549a384eec67572ee05c31621b8b4174425ab3))


### Bug Fixes

* **macos:** remove AXPress from dismiss action list ([27ef4f3](https://github.com/lahfir/agent-desktop/commit/27ef4f34c038c26f5852bf1f6026762a98d0df0a))
* **macos:** restore frontmost app after notification center interaction ([3881bc8](https://github.com/lahfir/agent-desktop/commit/3881bc82bdb5a4bb7689f1e1e2237bb745e60c21))
* **macos:** use pgrep and async osascript for NC lifecycle ([9797585](https://github.com/lahfir/agent-desktop/commit/979758538fca0145e9245641fb03a0769eed68de))

## [0.1.6](https://github.com/lahfir/agent-desktop/compare/v0.1.5...v0.1.6) (2026-02-24)


### Bug Fixes

* handle null bounds in refmap and improve sidebar click resolution ([d4197e8](https://github.com/lahfir/agent-desktop/commit/d4197e8f6f6700f2f672d3e1e436ecf24cf82e01))

## [0.1.5](https://github.com/lahfir/agent-desktop/compare/v0.1.4...v0.1.5) (2026-02-23)


### Features

* add fallback chains for set-value, clear, focus, scroll-to, type and post-action state hints ([11f8da0](https://github.com/lahfir/agent-desktop/commit/11f8da06e84ed67b0e26dbc1946f7a7542e89dcd))
* add structured verbose logging across all layers ([c7316e8](https://github.com/lahfir/agent-desktop/commit/c7316e8b5160ab0e6ba554bcd502d4c47adf8b1a))


### Bug Fixes

* add dwell time before drag release for drop target recognition ([2a52d62](https://github.com/lahfir/agent-desktop/commit/2a52d62106699b36f62f9af83895f2264b80efb1))

## [0.1.4](https://github.com/lahfir/agent-desktop/compare/v0.1.3...v0.1.4) (2026-02-23)


### Features

* add agent-desktop skill for universal AI agent support ([ef45135](https://github.com/lahfir/agent-desktop/commit/ef45135087d09a7e065f65d9a0558d1e710cb8bf))
* add Claude Code skills for agent-desktop automation ([ad91cd3](https://github.com/lahfir/agent-desktop/commit/ad91cd32cf2de1c1c8dcda4c0dcae37f0022b4c6))

## [0.1.3](https://github.com/lahfir/agent-desktop/compare/v0.1.2...v0.1.3) (2026-02-23)


### Bug Fixes

* correct GitHub Release download URL and simplify tag format ([8f66a93](https://github.com/lahfir/agent-desktop/commit/8f66a9346e02a751a83ac02313dcec2d9c81bde8))
* include README and CHANGELOG in npm package ([084fc8c](https://github.com/lahfir/agent-desktop/commit/084fc8c960527c0d0654028794ec3c4fd2d970c4))


### Performance

* use curl for binary download in postinstall ([ebafb71](https://github.com/lahfir/agent-desktop/commit/ebafb71603f5f2b32af8ac5bf6c88df3d6012f70))

## [0.1.2](https://github.com/lahfir/agent-desktop/compare/agent-desktop-v0.1.1...agent-desktop-v0.1.2) (2026-02-23)


### Bug Fixes

* use macos-latest for both build targets ([91c7677](https://github.com/lahfir/agent-desktop/commit/91c76777cb7ee864b45e14d123c79c08f0c2d5b9))

## [0.1.1](https://github.com/lahfir/agent-desktop/compare/agent-desktop-v0.1.0...agent-desktop-v0.1.1) (2026-02-23)


### Features

* 10-step scroll chain, focus guards, enhanced click chain, bounds fix ([595ccb6](https://github.com/lahfir/agent-desktop/commit/595ccb6cc45554351ea3e30b95e4ca47bdf4e16b))
* add 19 new commands, AX-first rewrites, LOC compliance ([d3f7e03](https://github.com/lahfir/agent-desktop/commit/d3f7e03c67832c652a6125f61fbb7ab2f0801939))
* add 19 new commands, AX-first rewrites, LOC compliance ([eca04e8](https://github.com/lahfir/agent-desktop/commit/eca04e839288b121f6f41c6de525a8396d10654c))
* add release automation with GitHub Releases and npm distribution ([18fc50c](https://github.com/lahfir/agent-desktop/commit/18fc50cca51f2ed10b6dfb5576602b6ce344bc95))
* add structural hints to splitter columns in snapshots ([48f8470](https://github.com/lahfir/agent-desktop/commit/48f8470948b4f636dfa6f4489e4cb6d9f520722c))
* AX-first right-click chain with inline context menu capture ([cddc5d3](https://github.com/lahfir/agent-desktop/commit/cddc5d3547f058a78f8b398fa982e39a1fcbf6b1))
* Phase 1 foundation — workspace scaffold, core engine, macOS adapter, 31 commands ([a346f24](https://github.com/lahfir/agent-desktop/commit/a346f242c25dfad1c849e6d50f9ab25a42b462d9))
* smart AX-first click chain + macOS crate restructure ([4616c8f](https://github.com/lahfir/agent-desktop/commit/4616c8f65f974505b0eedb5485c865d3b905342b))
* surface-targeted snapshot, menu wait, list-surfaces command ([39178b2](https://github.com/lahfir/agent-desktop/commit/39178b291602d192de97aa0150c261db1dcc7ca6))


### Bug Fixes

* add menubar surface, fix press --app crash and modifier mapping ([a231962](https://github.com/lahfir/agent-desktop/commit/a2319623b4d1d2b6b2f6e1a4ab9a8b8cbbfd02eb))
* address code review findings (double-free, CF leaks, injection) ([2f495ff](https://github.com/lahfir/agent-desktop/commit/2f495ffb69be67f3136b076534e078cc31b005c2))
* align error codes with spec (APP_NOT_FOUND, PERM_DENIED) and add -i shorthand ([6dc567a](https://github.com/lahfir/agent-desktop/commit/6dc567a4aedff15cf82a82601089cb0b87da4e26))
* ancestor-path cycle detection + CGEvent click fallback ([198d7d7](https://github.com/lahfir/agent-desktop/commit/198d7d7d27167044a448b6616fa5c9c0554321bf))
* detect open menus via AXMenuBarItem.AXSelected, not AXMenus attribute ([7f0d610](https://github.com/lahfir/agent-desktop/commit/7f0d6103d16969a0abfa84a62b6819dbd0d1cc8e))
* make all 30 commands work end-to-end on macOS ([1d98ab8](https://github.com/lahfir/agent-desktop/commit/1d98ab828ce5bcb39e212548ae2f2a052e67aac9))
* remove AXShowDefaultUI from activation chain, fix child walk ([74242f5](https://github.com/lahfir/agent-desktop/commit/74242f5040af9c46c98a3f5232dc7567538c28e1))
* resolve all 47 code review findings from Phase 1 audit ([218503a](https://github.com/lahfir/agent-desktop/commit/218503a7ebacacd4fbc6b388a6cf5e3bb86af039))
* right-click uses AXShowMenu; context menus detected via focused element ([2c9aee3](https://github.com/lahfir/agent-desktop/commit/2c9aee397912d6a903d9ef1e26c786697383ae95))
* suppress dead_code lint on BatchCommand deserializer struct ([608d4aa](https://github.com/lahfir/agent-desktop/commit/608d4aaaa195b95626f17aa4bbca2d69609f14cc))
* use simple release strategy for workspace version bumps ([0ab78dd](https://github.com/lahfir/agent-desktop/commit/0ab78dde0e1ff702db6c8b667784fa456245b26b))
