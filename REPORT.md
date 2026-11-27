# REPORT — review of `mashtab-bim` (Масштаб AI / BIM), reviewer Claude Opus 5.5

VERDICT: MERGE AFTER COORDINATOR CHECK

Branch `wendamys/mashtab-bim-review`, HEAD of the checked code 9f0aca15 (this `report:` commit adds only REPORT.md). Merged: `origin/mashtab-bim` bb334e5f (author `report:`, parts
ae474622, d394f2a0, 09fe92fd, d2e462e7, d4908c9f), which already contains `origin/wendamys/pdf-extend-review` ad8f680c
(3e6e744f; `git merge-base --is-ancestor` true). Reviewed diff: `origin/wendamys/pdf-review-b...origin/mashtab-bim`,
brand part `68bf2cc4..bb334e5f`; requirements: criteria 1–7 of `coord/brand.md` + `coord/brand-inbox.md`.

## Findings
1. `crates/cc_app/src/update.rs:263` (author's line 260) — `if !ENABLED || disabled_by_env() || cfg!(test)`: in tests
   `cfg!(test)` alone returned Disabled, so no test held the ENABLED gate of manual checks (found by reading). Fixed in
   6d99fa80: the test-offline guard moved into `spawn()` (`update.rs:170`, disconnected receiver = failed check). Mutation
   M2 (drop `!ENABLED ||`) after the fix: FAIL `manual_checks_and_installation_are_disabled` (`!updates.checking()`, update.rs:516) — held. Before the fix the same mutation is masked by `|| cfg!(test)` (read from code; not run separately).
2. nit `crates/cc_app/src/dialogs.rs:406,449` — About's "Repository" row shows upstream `CARGO_PKG_REPOSITORY`
   (github.com/francisreboucas/constructcraft) while the "Open repository" button opens the fork. Left: both are true
   (upstream project, fork source above); product choice for the coordinator.
3. nit `crates/cc_app/src/ribbon.rs:990` — "МАСШТАБ AI" is a hard-coded string, not an ftl key (brand wordmark, same in
   every language); `about-familiar` key is now unused. Left.
4. Six snapshot tests fail with `CC_UI_SNAPSHOTS=1` on this machine (home, preferences, ribbon — as the author — plus
   graphics_dialog (`filters`, 98 882 px), icon_gallery (`icons_*`, ~34 000 px), library (`library_*`, 274–793 px), which the
   author did not run). All on references the branch does not change, rendered by code the branch does not change
   (`git diff --stat 68bf2cc4 HEAD -- assets/icons/ui crates/cc_core crates/cc_app/src/{icons,library,home,palette}.rs
   examples` and those PNGs — empty). Seen: `filters.new.png` has a category the reference lacks («Marcadores de elevação
   interna»), `icons_*.new.png` has more icons — stale upstream references. Masks: `.tmp/review/drift/`. Not fixed.
5. Test hermeticity (pre-existing, not branch): `home_snapshots` reads sample thumbnails from the real profile
   (`thumbnails.rs:24,38`, `storage_dir("ConstructCAD")/thumbnails/sample-*.png`). On office-65 the cache
   `C:\Users\stepan-orca\AppData\Roaming\ConstructCAD\data\thumbnails\sample-casa_simples.png` (25.11 21:03) is the hip-roof
   house with a canopy — exactly the "other house" in the author's `home_*` drift. Not fixed (outside the task).

## Unanchored guarantees
- `app.rs:94`, `app.rs:492` `update::ENABLED &&` before `remove_old_executables` / `update_install::finish`: startup and
  exit of the eframe app, no test. `on_exit` is only set by `update_now` (M4 not run).
- `plan()` gate `update.rs:234` (M3 not run) — `update_now` has its own gate; `plan` is also read by the dialog only when
  `available` is set, which only a network answer sets.

## Base vs branch
- Committed references (`git show 68bf2cc4:` vs HEAD, RGB diff): ribbon_architecture / ribbon_file 1100 px differ only in
  x 10–580, y 9–31; ribbon_narrow 520 px only x 10–489, y 9–31; preferences_general only the Updates group (300–912 ×
  233–403). Everything else in those frames is pixel-identical to the base.
- Drift of six untouched references (author's 3 failing tests): home_* — profile thumbnail cache (finding 5, shown by
  the cache file). palette (1-px line y=101), preferences_shortcuts (text cursor x 329–331), preferences_interface(_dark)
  (1–2 px of the light/dark split in the "System" tile): rasterisation-level; the branch does not touch palette.rs,
  home.rs or the Interface page. A run of the same tests on base 68bf2cc4 was not done (time; a second worktree means a
  full rebuild on the HDD): cause for these four — not determined, passed to the gate.

## Probes for the coordinator
1. `git -C <wt> grep -n "ENABLED" crates/cc_app/src/update.rs crates/cc_app/src/app.rs` — 6 gates.
2. `git diff 68bf2cc4 HEAD --stat -- crates/cc_app/src/home.rs crates/cc_app/src/palette.rs examples` — empty.
3. Open `crates/cc_app/tests/snapshots/ribbon_narrow.png` — brand block and quick access in one row at 520 px.

## Run
Build dir `D:\StepanOrca\build\target-i18n`, `-j2`, via `one-build.ps1` (logs `.tmp/review/t1.log`, `run3.log`).
- Before the author's last part, HEAD with fix: `cargo nextest run -p cc_app update:: startup_language window_title title_` —
  54 passed (incl. update::tests::*, preferences startup_language, app title tests).
- HEAD 9f0aca15 (+ temporary `review_brand_shots`, not committed): `CC_UI_SNAPSHOTS=1 cargo nextest run -p cc_app
  review_brand_shots snapshot ribbon_buttons_are_accessible every_dialog_keeps update:: startup_language window_title` —
  22 run, 16 passed, 6 failed (snapshot drift, finding 4); passed: all update::tests, startup_language, window title,
  ribbon_buttons_are_accessible, every_dialog_keeps_its_body_inside, phases/sheet/title-block snapshots.
- `cargo nextest run -p cc_engine --lib i18n::` — 8/8 passed (application_name_is_shared_by_localized_messages,
  languages_are_complete, placeholders…).
- Mutations (applied by `.tmp/review/mutate.py`, reverted after each, `git diff --quiet` checked):
  | mutation | failed | added |
  |---|---|---|
  | M2 `start`: drop `!ENABLED \|\|` | 1 (`manual_checks_and_installation_are_disabled`) | fix 6d99fa80 |
  | M1 `auto_check_due`: drop `ENABLED &&` | 1 (`automatic_checks_are_disabled_for_all_preferences`) | — |
  | M5 `startup_language`: Ru → from_system | no result: started 17:54, the run was stopped by Claude Code for low system memory (~18:05); mutation reverted by hand, tree clean | — |
  | M3/M4/M6 (plan gate, update_now gate, saved language) | not run | — |
- `cargo clippy -p cc_app --all-targets -- -D warnings` (clean tree): PASS (exit 0, 17:51–17:54). `cargo fmt --all -- --check`: PASS on HEAD.

## Сверка с пакетом
| Требование | Улика | Есть/нет |
|---|---|---|
| 1 Значок: png/256/ico 16…256 из логотипа, прозрачный, «М» по центру; exe | `assets/icons/*` (my sheet `.tmp/review/icon-sheet.png`, ico frames 16/24/32/48/64/128/256 RGBA), `build.rs:12` set_icon, `main.rs:146` window icon | есть |
| 2 Лента: знак + МАСШТАБ AI (Russo One) + «/» + BIM, обе темы, узкая | `ribbon.rs:978-995`, `theme.rs:249,335`; `tests/snapshots/ribbon_*.png`; my shots `.tmp/shots/ribbon_{ru,en}_{Light,Dark}_{1600,1100,620,619,520,400}.png` | есть |
| 2 Russo One + OFL рядом, упоминание | `assets/fonts/RussoOne-Regular.ttf`, `OFL-RussoOne.txt`, `README.md:276`, `about-russo-credit` | есть |
| 3 Название: окно, «О программе», ftl; не менять ccad/.ccad/каталог настроек | `cc_engine/src/app_name.rs:2`, `i18n.rs:154` + test `application_name_is_shared_by_localized_messages`, `app.rs:51`, `main.rs:146`; `recovery.rs:22`, `thumbnails.rs:24`, `main.rs:167` `ConstructCAD` kept; remaining ConstructCraft in ftl = upstream credit/independence/dead update-* strings | есть |
| inbox: «Масштаб AI / BIM», заголовок «… — <проект>» | `app_name.rs:2`, `app.rs:51` test `the_window_title_names_the_open_file` | есть |
| inbox: «О программе» — описание ru/en, «сборка Масштаб AI 1 от 27.11.2026» | `dialogs.rs:422-432`, `ru.ftl` about-text/about-brand-build, `en.ftl:745` | есть |
| inbox: свойства exe (4 поля) | `build.rs:13-19` verbatim; VersionInfo checked by author on a probe exe (not by me) | есть |
| 4 GPL: строка, исходники, пометка изменённой версии с датой, лицензия | `about-modified-build` (ru/en, others English), `dialogs.rs:407,432` URL `…/tree/mashtab-bim`, `LICENSE` unchanged | есть |
| 5 Обновления: нет запроса/скачивания; UI убран/«отключены»; тест «нет» при любых настройках | `update.rs:22,140,234,263,313`, `app.rs:94,492`; tests `automatic_checks_are_disabled_for_all_preferences` (M1 FAIL at update.rs:460), `manual_checks_and_installation_are_disabled` (M2 FAIL at update.rs:516 after fix 6d99fa80), `ribbon_buttons_are_accessible`; backstage/About button removed, `preferences.rs:726` | есть |
| 6 Язык: без сохранённого — ru, сохранённый — как раньше | `preferences.rs:427` + test `startup_language_prefers_saved_language_to_russian_default` (M5 not run, M6 not run) | есть |
| 7 Имя exe | nothing at runtime depends on `constructcraft.exe`: `mcp_bridge.rs:19` ccad beside current exe, `gpu.rs:185` current_exe, `update_install.rs:308` dead; `packaging/windows/*.wxs` not used for the folder | есть |

## Визуальная проверка
| Кадр | Кто смотрел | Ожидаемое → увиденное | Вердикт |
|---|---|---|---|
| `.tmp/review/refs/cmp_ribbon_architecture.png` (base over branch, ref 1100 px ×2) | Claude, ревьюер | блок как у сметчика → знак «М», «МАСШТАБ AI» Russo One, приглушённая «/», BIM, затем кнопки; как `smetchik-header.png` | PASS |
| `.tmp/review/refs/cmp_ribbon_narrow.png` (520 px) | Claude, ревьюер | узкая лента не ломается → блок и быстрый доступ в одном ряду, кнопки уже, ниже y=31 кадр = база | PASS |
| `.tmp/review/refs/cmp_prefs.png` (preferences_general) | Claude, ревьюер | переключатель обновлений → текст «отключены», остальное = база | PASS |
| `.tmp/review/refs/cmp_pref_iface.png` (drift) | Claude, ревьюер | не ветка → 1–2 px на плитке «Система» | n/a (дрейф) |
| `.tmp/shots/about_ru_Light.png`, `about_ru_Dark.png` | Claude, ревьюер | крупно «Масштаб AI / BIM», описание, «сборка Масштаб AI 1 от 27.11.2026», GPL с датой, исходники → всё видно, заголовок и кнопки в кадре, тело прокручивается | PASS |
| `.tmp/shots/about_en_Dark.png`, `about_en_Light.png` | Claude, ревьюер | то же по-английски → «Building design in BIM…», «Масштаб AI build 1, 2026-11-27», GPL, URL форка | PASS |
| `.tmp/review/refs/cmp_myribbons.png` (ru 1600 dark, 520/620/619 light, 400 dark) | Claude, ревьюер | блок читается в обеих темах; узкая не ломается → 1600/620/520 — один ряд; 400 — быстрый доступ переносится на 2-й ряд (окно не уже 900 px, `main.rs:145`) | PASS |
| `.tmp/review/refs/cmp_icons.png`, `cmp_filters.png` (drift) | Claude, ревьюер | не ветка → новые значки и категория в коде базы, эталоны базы устарели | n/a (дрейф базы) |

## Передано в гейт
- Mutations M3 (`update.rs:234` plan gate), M4 (`update.rs:313` update_now gate), M5 (`preferences.rs:428` Ru default →
  from_system), M6 (saved language ignored) — script `.tmp/review/mutate.py M3|M4|M5|M6` + `cargo nextest run -p cc_app
  update:: startup_language`; expected: M4/M5/M6 fail, M3 may pass (update_now has its own gate).
- Consumers of `cc_engine::i18n` (author's list): `cc_engine --test main` bar_marks, imperial_text, macro_editor,
  material_libraries, pipe_fittings, pipe_tags, schedule_def, title_block_migration; `cc_plugins --test main` api2, files.
- Snapshot drift of 6 tests (finding 4) on base 68bf2cc4 on this machine — cause of palette/preferences drift not determined;
  home_* is the profile thumbnail cache. Decide: refresh references in a separate change or keep as is.
- Built exe VersionInfo (author checked a probe exe only).

## Incidents and cleanup
- ~16:2x a `orca orchestration check` with a PATH prepend resolved to the GUI `D:\StepanOrca\app\orca\Orca.exe`; it hung
  120 s and was stopped; afterwards only the 15:21 Orca server process existed (whether a window flashed — not known).
- 16:30–16:37 my run held the build slot; stopped at the coordinator's 16:36 message (author first), no mutation applied.
- ~18:05 Claude Code stopped run3 (low system memory) during M5; mutation and temporary test reverted by hand, 4 of my
  leftover `tail` monitors stopped by PID; build slot free, tree clean.
- Cleanup: deleted 0 files; moved 26 kittest `.new/.diff` PNG from `crates/cc_app/tests/snapshots` to `.tmp/review/drift/`.
  Kept in `.tmp/` (worktree): `review/` scripts, logs, refs and crops, `shots/` (28 PNG), `findings.md`, `report-draft.md` —
  evidence for this report; remove with the worktree after acceptance (coordinator).
