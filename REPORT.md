VERDICT: MERGE AFTER COORDINATOR CHECK — найдены и исправлены 2 дефекта PDF в A и латинские «A/VA» в B; гейт B перезапустить на 68bf2cc4; ниже риск общего CARGO_TARGET_DIR.

## Находки (A = d39a31c3 + 7c330c65/37d1c080; B = 59e01d92 + cf1b3f95; ревьюер claude opus-5.5)
- A export.rs:178 (d39a31c3) — 16 знаков WinAnsi без байта в таблице (`™ Œ ‰ ƒ † …`) оставались «?», хотя в Inter есть глифы; это против решения Степана. Найдено чтением, тест `pdf_keeps_unmapped_win_ansi_symbols` был красным. Исправлено в 98264ebe: ветка с подстановкой убрана. Латинские документы без этих знаков — байт-в-байт.
- A export.rs:438 — предикат `needs_inter` расходился с `to_pdf`: строку только из U+2009/U+2003 `to_pdf` пропускает, а `layout_on_paper` по ней расширял страницу (22,54 вместо 22 мм; тест `blank_text_keeps_pdf_paper` был красным). Исправлено: проверка `trim` перенесена в предикат.
- п.1 доп. (7c330c65, 37d1c080) — CIDToGIDMap и `/W` не закрывал ни один тест; знак смещения якоря в `layout_on_paper` тоже (мутация E11 выжила). Дописаны `pdf_codes_select_inter_glyphs_and_widths` и `aligned_wide_text_stays_on_pdf_paper` (aaa3ac90). Код верен. Ссылки `file:line` прежнего отчёта сверены, совпадают; неточность одна: `macro_editor.rs` лежит в `crates/cc_engine/tests/`. cf1b3f95: `format::size(…Mm(0))` равен прежнему тексту в metric и imperial, тест `cli.rs::calculated_schedule_units…` проходит.
- п.2 B — латинские «A/VA» под ru в diagram.rs:59,80,85; тот же класс — mep.rs:474-475 (марка розетки), schedule.rs:133 (ячейка мощности), cc_app systems_dialog.rs:246-248,417-418, properties.rs:1741. Добавлены `"A"→"А"` и `"VA"→"ВА"` в `russian_symbol`, вывод идёт через `unit_symbol` (0a888262). Три теста были красными до правки. en — байт-в-байт.
- п.3 B ru.ftl — заголовки: «Напряжение (В)» 17,34 мм при месте 14, «Проводник (мм²)» 18,58 при 18 (Inter 2,2 мм, мой замер) → «Напряж. (В)» 12,81, «Сечение (мм²)» 15,90 (96cb7582).
- Не чинил, вне задачи: те же колонки переполнены и в других языках — de «Schutzschalter (A)» 19,26 при 16, es 26,99, it 33,08, uk «Навантаження (ВА)» 20,96 при 20, en/es «Conductor (mm²)» 18,05 при 18. Большое пустое поле на PDF/SVG-странице вида 347 есть и в main (старая квадратная оценка текста в `layout_on_paper`).
- РИСК ГЕЙТА: деревья в одном `CARGO_TARGET_DIR` получают одинаковые хеши юнитов, свежесть cargo берёт по mtime. Мои сборки `git archive` main и 1500cbdc (mtime старше артефактов) выдали бинарник ветки: md5 трёх `ccad.exe` совпал (af595cf4…). Гейт дерева со старыми mtime может прогнать чужой код. Лечится `touch` дерева или отдельным target.

## Мутации (запуском, nextest; логи `.tmp/mut-*.log`)
| что сломали | упало | что дописал |
|---|---|---|
| всегда Inter / только кириллица / без проверки глифа / границы по старому предикату | 4 / 2 / 1 / 1 | — |
| без `trim` / вернуть `™ Œ`→`?` / CIDToGIDMap без слота 0 / CID с 0 / общий код ʼ и ’ | 1 / 0→1 / 1 / 1 / 1 | `pdf_keeps_unmapped_win_ansi_symbols`, `pdf_codes_select…` |
| знак якоря в границах / без конца строки / PDF-вид по SVG-раскладке (io.rs) | 0→1 / 1 / 1 (`drafting::a_wide_cyrillic_note_fits_its_pdf_page`) | `aligned_wide_text_stays_on_pdf_paper` |
| латинские A, A, VA в diagram / VA в mep, schedule / нет `"A"→"А"` / болт «mm» (cf1b3f95) | 1 / 1 / 1 / 1 / 1 / not_tested (3 обрыва: 0xC0000142, перезагрузка, timeout 25 мин 15:28–15:53) | тесты в diagram, mep, schedule |

## База против ветки (showcase f32032c2…, листы 83, виды 82 и 347; мои сборки main, 1500cbdc, ветки)
- en, pt-BR (PDF+SVG, 18 файлов): ветка = main побайтно. zh-CN, ja, lo (9 PDF): ветка = 1500cbdc побайтно (у zh-CN rooms по 10 «?» во всех трёх).
- ru 347: main — 105 «?», 1500cbdc — 1 «?» («?U», «Ввод 63 A», «VA»), ветка — 0: «ΔU (%)», «Ввод 63 А», «500 ВА». Копия с «Œuvre™ room ‰»: main и 1500cbdc — по 3 «?», ветка — текст целиком.

## Прогон
- cc_views lib `export:: diagram:: mep:: schedule:: format::` — 43/43; cc_engine `--test main` (31 модуль-потребитель, `.tmp/consumers.txt`) — 278/278; cc_engine lib `i18n::` — 7/7; cc_cli `--test cli` — 17/17.
- `cargo fmt --all -- --check`, `cargo clippy -p cc_views -p cc_engine -p cc_app --all-targets -- -D warnings`, `git diff --check` — PASS. Мутации C1 (болт «mm», cf1b3f95) и C2 (bending «kg») не прогнаны; C1 закрывает `cli.rs:106` `("ru", "Ø 20 мм", …)` по чтению, запуском не подтверждено. Бин-тесты cc_app не запускал.
- Upstream main = abf32c24, уже в обеих ветках (fetch 14:20). A `wendamys/pdf-extend-review-a` = aaa3ac90, B `wendamys/pdf-extend-review` = 68bf2cc4; обе запушены. Проверены диапазоны 1500cbdc..d39a31c3, eb646f85..59e01d92, 7c330c65, 37d1c080, cf1b3f95.

## Визуальная проверка
| кадр | кто смотрел | ожидаемое → увиденное | вердикт |
|---|---|---|---|
| .tmp/shots/branch-ru-diagram.png | claude opus-5.5 | ru 347: «ΔU (%)», «А/ВА» кириллицей → «Ввод 63 А», «6 А», «500 ВА», «ΔU (%)» | PASS |
| .tmp/shots/branch-ru-diagram-table.png | claude opus-5.5 | шапка в колонках → «Напряж. (В)», «Сечение (мм²)» внутри, справа просвет | PASS |
| .tmp/shots/branch-tm-rooms-detail.png | claude opus-5.5 | «Œuvre™ room ‰» без «?» → читается целиком | PASS |

## Передано в гейт
- crates/cc_app/src/systems_dialog.rs, crates/cc_app/src/properties.rs (бин-тесты cc_app не гонял)
