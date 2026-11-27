# Масштаб AI / BIM

Задача mashtabai/stepan-tasks#208. Ветка `mashtab-bim` в `wendamys/constructcraft`. При сверке использовано написание из `D:\StepanOrca\coord\brand-inbox.md`. После двух обрывов Orca продолжена существующая ветка с коммитами `ae474622`, `d394f2a0`, `09fe92fd` и слиянием `e925ca63`. Координатор перенёс оценку сдачи с 16:20 на 16:35 (+07) 27.11.2026. Сборка проверок стартовала в 15:57, когда координатор уточнил, что лимит одной сборки применяется к процессам работника.

## Критерии

| № | Результат и проверка |
| --- | --- |
| 1 | `assets/icons/app-icon.png` (512²), `app-icon-256.png` (256²) и `app-icon.ico` из утверждённого RGBA PNG, SHA-256 `f19df93c6066f8ab0f8f86d82641c3f74b81b56e80314db58ac26e18eeb48599`. `.tmp/brand_check_assets.py` подтвердил прозрачность, центрирование и кадры ICO 16/24/32/48/64/128/256. Проверочный лист `.tmp/shots/icon_sizes.png` просмотрен. `main.rs:146`, `theme.rs:224`, `build.rs:12` задают значок окна, интерфейса и Windows-ресурс. |
| 2 | Блок `МАСШТАБ AI / BIM` в `ribbon.rs:978` с фирменным знаком и отдельным семейством Russo One (`theme.rs:249`, `theme.rs:335`). SHA-256 шрифта `bc0abcc660bd8b7ad3000ecb2898a27c58a29a50f7ec81652fa12e75148d09df` совпал с Google Fonts. Текст OFL совпал после нормализации концевых пробелов. Лицензия перечислена в `README.md:276` и `README.pt-BR.md`. |
| 3 | Общая константа `cc_engine/src/app_name.rs:2`, подстановка `app-name` в `cc_engine/src/i18n.rs:154`, начальный заголовок `main.rs:146` и заголовок проекта `app.rs:51`. Сохранены eframe ID `ConstructCAD` и Linux app ID `constructcraft`, расширение `.ccad`, CLI `ccad` и пути восстановления (`recovery.rs:22`). Сборочный скрипт `cc_app/build.rs:12` задаёт все четыре согласованных поля exe. PowerShell `VersionInfo` на `.tmp/resource-probe/resource-probe.exe` подтвердил FileDescription и ProductName `Масштаб AI / BIM`, CompanyName `Масштаб AI`, LegalCopyright `© Масштаб AI · mashtab-ai.ru; основано на ConstructCraft © Francis Rebouças, GPL-3.0`. |
| 4 | «О программе» в `dialogs.rs:402` показывает знак, название, согласованный текст ru/en, номер релиза и `сборка Масштаб AI 1 от 27.11.2026`, дату изменения, GPL, автора и ссылку на ветку форка. Сохранены `LICENSE`, `assets/icons/FLATICON.txt`, исходные кредиты и ссылка на репозиторий ConstructCraft. |
| 5 | `update.rs:22,139,227,260,309`, `app.rs:492`: автоматическая и ручная проверки привязаны к `ENABLED = false`, ручная возвращает `Disabled` до построения запроса, скачивание/установка и запуск установщика на выходе закрыты. Из File убрана кнопка проверки (`backstage.rs`), команда и общие настройки выводят «Обновления в этой сборке отключены». |
| 6 | `preferences.rs:423`: `--lang` → сохранённый код → русский. До загрузки сохранённой настройки ошибки старта получают `--lang` или русский (`main.rs:154`). |
| 7 | Для выдаваемой папки можно назвать программу `Mashtab AI BIM.exe`. `mcp_bridge.rs:19` ищет `ccad.exe` рядом с текущим exe, `gpu.rs:185` повторно запускает фактический путь, а ресурсы вшиты и настройки привязаны к постоянным ID. В рабочем пути приложения нет проверки имени процесса для одного экземпляра. В upstream MSI ассоциация `.ccad` и ярлыки ссылаются на `constructcraft.exe` (`packaging/windows/constructcraft.wxs`), upstream workflow упаковывает бинарник под исходным именем (`.github/workflows/release.yml:69`), отключённый updater ищет это имя внутри архива (`update_install.rs:308`). При упаковке портативной папки оставить `ccad.exe` рядом, ярлык направить на переименованный exe. |

## Ключи ftl

`about-brand-build`, `about-icon-credit`, `about-modified-build`, `about-russo-credit`, `about-source-code`, `about-text`, `app-name`, `clipboard-note`, `cmd-desc-check-updates`, `cmd-desc-open`, `dlg-recovery-text`, `err-image-unreadable`, `err-parse`, `file-filter`, `file-filter-component`, `file-filter-macro`, `gpu-failed-title`, `plug-incompatible`, `plug-needs-app`, `plug-review`, `update-disabled-in-build`. `err-image-unreadable` сохранён в ru и uk-UA, где формулировка уже была нейтральной. 13 каталогов совпали по ключам и плейсхолдерам в проверке `.tmp/brand_check_assets.py`.

Русские и английские строки сверены с решением Степана. Русское описание — «Проектирование зданий в BIM: планы, 3D-модель, разрезы, листы и спецификации». Английское — «Building design in BIM: plans, 3D model, sections, sheets and schedules».

## База и проверенный код

- Влита `origin/wendamys/pdf-extend-review` на `ad8f680c6b65dec21e68d1ea5ca491fb0740cbc8` коммитом `3e6e744f`. Обновление базы добавило только отчёт PDF-ревью. Отчёт PDF-ревью сохранён в исходной ветке.
- `d2e462e7` подтягивает интервалы фирменного блока на узкой ленте. Сборка приложения выполнялась с этим исходным кодом и с временным подключением `.tmp/brand_ui.rs` для проверки снимков.

## Выполненные проверки

- `cargo fmt --all -- --check`, `git diff --check 68bf2cc4` — PASS на исходном коде.
- `.tmp/brand_check_assets.py` — PASS. Независимый повтор преобразования из master дал те же RGBA-пиксели обоих PNG и побайтово тот же ICO.
- `VersionInfo` на `.tmp/resource-probe/resource-probe.exe` — PASS. Cargo также создал `resource.rc`, `resource.o` и `libresource.a` в `target-brand-review` при сборке `cc_app`.
- Автонастройки, обновление, версия, название и ссылка сверены с кодом и со всеми 13 каталогами. [Russo One в Google Fonts](https://github.com/google/fonts/tree/main/ofl/russoone).

## Передано в гейт

- `cargo clippy -p cc_app --all-targets -- -D warnings`.
- Локализация `cargo nextest run -p cc_engine --lib i18n::`, потребители `cc_engine/tests/{bar_marks,imperial_text,macro_editor,material_libraries,pipe_fittings,pipe_tags,schedule_def,title_block_migration}.rs` через `--test main` и `cc_plugins/tests/{api2,files}.rs` через `--test main`. Сборка проверок приложения заняла доступное время. Скрипт `.tmp/brand_cargo.ps1` задаёт окружение и вызывает `one-build.ps1` для команд, требующих слот.

## Визуальная проверка

| Кадр | Кто смотрел | Ожидаемое → увиденное | Вердикт |
| --- | --- | --- | --- |
| `.tmp/shots/icon_sizes.png` | Codex, автор | Исходный «М» в малых размерах → узнаваемый силуэт, форма и цвета совпали с утверждённым знаком | PASS |
| `ribbon_520_{light,dark}_ru.png` | — | Бренд и меню быстрого доступа в одном ряду → кадр ещё не получен | not_tested |
| `ribbon_1600_{light,dark}_ru.png` | — | Читаемый фирменный блок на широкой ленте → кадр ещё не получен | not_tested |
| `about_ru.png`, `about_en.png` | — | Описание, версия, дата, GPL, ссылка на исходники → кадры ещё не получены | not_tested |

Прогон `UPDATE_SNAPSHOTS=1 cargo nextest run --locked -p cc_app --bin constructcraft -E 'test(/capture_brand_ui|ribbon_snapshots|preferences_snapshots|home_snapshots|narrow_ribbon_scrolls|ribbon_compacts_before_scrolling|ribbon_buttons_are_accessible|preferences_open_from_the_file_tab_and_the_shortcut|every_dialog_keeps_its_body_inside|automatic_checks_are_disabled_for_all_preferences|manual_checks_and_installation_are_disabled|startup_language_prefers_saved_language_to_russian_default|the_window_title_names_the_open_file|an_unchanged_title_lets_the_app_settle|mcp_arguments_go_to_ccad/)' --no-capture --no-fail-fast` дошёл до линковки `constructcraft-9eae4f2fe9a69ceb.exe`. В первой попытке исправлено время жизни локализованной подписи во временном тесте (`E0716`), вторая прошла компиляцию Rust. Прогон остановлен перед сдачей в 16:33. Линковка на HDD оставалась незавершённой, размер exe составлял 0 байт. Результат целевых тестов, обновление эталонов и визуальная приёмка ленты и «О программе» остаются открыты.

Для повторной съёмки подключить `.tmp/brand_ui.rs` временным `#[path = "../../../.tmp/brand_ui.rs"] mod brand_ui;` из `crates/cc_app/src/ui_tests.rs`. Возвращаемые эталоны — в `crates/cc_app/tests/snapshots`, дополнительные PNG — в `.tmp/shots`. Готовую папку portable и финальный exe проверяет координатор при упаковке.

Уборка. Удалены две временные загрузки шрифта и лицензии и ранний скрипт ожидания, 44 688 байт. Черновик отчёта удалён после переноса в REPORT.md (11289 байт). Сохранены `.tmp/shots`, `.tmp/brand_ui.rs`, `.tmp/brand_cargo.ps1`, `.tmp/resource-probe` и сборочный кэш `D:\StepanOrca\build\target-brand-review` для завершения проверки и воспроизведения.
