# Замечания переводчиков (из REPORT.md частей 1–4)

## Часть 1

# Отчёт: перевод части 1 (en.ftl 1–1160 → ru-work/parts/ru.1.ftl)

Итог `python -I ru-work/check_part.py 1`: строк 1160/1160, ошибок 0, предупреждений 41 — на HEAD
6b3fdbcb (перед коммитом отчёта). База `wendamys/ru-locale` = d4f42eb5 (merge: «Already up to date»;
в origin этой ветки нет, взята локальная). Предупреждения: латиница только в названиях форматов,
программ и авторских строках (GIF/BMP/WebP, ANSI/ARCH, egui, Magnific, SIL Open Font License,
Autodesk, SHA256SUMS); остальные — «длинно», разобраны ниже. Заголовок файла (строки 1–2) взят как
в заготовке `assets/i18n/ru.ftl` («ConstructCraft — Русский»), прочие комментарии — как есть.

## Новые термины
- Aligned (размер) → Параллельный; Ordinate → Ординатный; Continuous → Цепочка
- Framing (кровли) → Каркас (кровли); Covering and Framing → Покрытие и каркас
- Inverted-T section → Тавровое сечение; Beam on the footing → Балка на подошве
- Lateral / Vertical Justification → Привязка в плане / по высоте
- Split line (разреза) → Линия с разрывом; Segment at start/end → Отрезок в начале/конце
- Leader tick → Засечка; Elbow → Излом; Cut/Fill Volume → Объём выемки / насыпи
- Shading (материал) → Раскраска (как стиль Shaded); Satin → Полуматовая; Mortar → Раствор
- Seen from (подложка) → Взгляд; Decimals (единицы) → Точность; Circulation (группа) → Лестницы
- Datum (группа ленты, только кнопка «Ось») → Оси

## Сомнительные строки
- tpl-level-ground / tpl-level-first: Ground Floor / Level 1 → «Этаж 1» / «Этаж 2» (по-русски первый
  этаж — наземный; «Уровень 1» рядом с «Этаж 1» путал бы). Новые уровни — «Уровень { $n }».
- matclass-gypsum: Gypsum → «Гипсокартон» по глоссарию, хотя класс материала шире (гипс, штукатурка);
  предлагаю в глоссарии для класса «Гипс».
- tpl-piece-frame: Frame → «Рама» — общий ключ для дверей и окон (openings.rs:94), «Коробка» у двери не выразить.
- tpl-param-seat-height: SeatHeight → «ВысотаСиденья» (имя параметра без пробела, как в pt-BR).
- default-schedule-name: { $category } Schedule → «Спецификация: { $category }» (категория во мн. ч.).
- status-preselect / -cycle: только подстановки; «{ $category }: { $type }», «({ $index } из { $count })».
- tile-*: Ceramic (colonial/French/Roman/Portuguese) → «Керамическая (колониальная/…)» — названия
  бразильских видов черепицы, русских аналогов нет.
- zoom-fit-all «Все виды целиком», zoom-sheet-size «Реальный размер листа» — по смыслу, не проверено в UI.
- units-currency-hint: «e.g. $» → «например, ₽».

## Длинные надписи (не сократил)
- prop-flip-hand «Сменить направление открывания» (глоссарий), tool-floor-opening «Отверстие в перекрытии»
  (кнопка инструмента), loc-core-interior/exterior (глоссарий), prop-room-bounding, info-fire-rating,
  info-keynote, mat-thermal, phys-specific-heat, prop-head-height, prop-sill/col-sill, comp-kind-casework,
  section-structural «Несущие конструкции», tx-move-viewport.

## Осталось
- Ничего по части 1. Сборка/тесты Rust не запускались (по заданию). Временные файлы `.tmp/` удалены.

## Часть 2

# Часть 2 русского перевода (en.ftl 1161–2000) — отчёт

Файл: `ru-work/parts/ru.2.ftl`, переведены все 840 строк (разделы с Modify tools по Phases). Недоделанного нет.
`python -I ru-work/check_part.py 2`: строк 840/840, ошибок 0, предупреждений 36 (35 — длина, 1 — «CAD-программ»).
Получено на HEAD `fcdc6666` (до коммита отчёта); база `wendamys/ru-locale` = `d4f42eb5` смержена (Already up to date).
Самопроверка: 100 случайных строк (seed 20261126) перечитаны, исправлены 2 («щёлкните по ней», тавтология «схему щита … щита»).

## Новые термины
- Phase / Phasing → Этап / Этапы (строительства); Phase filter → Фильтр этапов; Demolish → Снести; Existing / New Construction → Существующее / Новое строительство
- Visibility/Graphics → Видимость/графика; Override Graphics → Переопределить графику; View Template → Шаблон вида
- Halftone → Полутон; Projection/Cut Lines → Линии проекции/разреза; Surface/Cut Fill → Заливка поверхности / в разрезе; vg-pattern → Образец (и для линий, и для заливок)
- Hide/Isolate (temporary) → Скрыть/изолировать; Isolate → Изолировать
- Analytical model → Расчётная модель; Pad footing → Фундамент под колонну; Ground beam → Фундаментная балка; Raft → Фундаментная плита
- Stirrups/Cross-ties → Хомуты/Шпильки; Splice stagger → Разбежка стыков; Projection into cap → Выпуск в ростверк; Trimming bars → Окаймляющие стержни; Steel schedule → Ведомость арматуры; Structural plan → План конструкций
- Plumbing → Сантехника (лента, дисциплина), в описаниях — «водопровод и канализация»; Plumbing Plan → План ВК; Electrical → Электрика; Electrical device → Электроустройство
- Circuit → Группа (по глоссарию); Power/Lighting circuit → Розеточная группа / Группа освещения; Distribution board → Распределительный щит; Breaker (столбец) → Автомат
- Valve → Арматура (категория, инструмент); Gate/Ball/Check/Globe → Задвижка/Шаровой кран/Обратный клапан/Вентиль; Elbow/Coupling/Reducer/Tee/Cross/Wye → Отвод/Муфта/Переход/Тройник/Крестовина/Косой тройник
- Sizing → Подбор сечений; Fixture units → Единицы стока; Run (MEP) → Участок
- Transaction-имена (`tx-*`) — отглагольные существительные («Соединение стен», «Снос»), как «Действие» в истории отмены
- Префиксы марок B/C/F → Б/К/Ф; шифры систем CW/HW/SAN/V/RW → В1/Т3/К1/К1в/К2 (ГОСТ 21.601, кроме вентстояка)

## Сомнительные строки
- tpl-sys-vent-code, V → «К1в»: отдельного шифра вентстояков в ГОСТ нет (часть К1); system-vent/tpl-sys-vent → «Вентстояк»
- prop-supply-weight, Supply Weight → «Вес (водопровод)»; sizing-weights, weights → «весов» (выводится «12,0 весов»): бразильские «pesos» (веса водоразбора), русского аналога нет
- col-drain-units, Fixture units → «Единицы стока»: выводится и единицей после числа («12,0 Единицы стока») — заглавная из-за заголовка столбца
- prop-cut-length, Cut Length → «Длина реза» (альтернатива «Длина заготовки»)
- prop-system-kind, Carries → «Назначение» (заголовок столбца со значениями «Холодная вода», «Розеточная группа»…)
- diagram-main, Main → «Ввод» (в коде «{ Ввод } 63 A» — «A» латиницей из кода)
- truss-pratt/howe/warren → «Пратт/Хоу/Уоррен» (имена схем ферм транслитерацией)
- tray-ladder, Ladder → «Лестничный» (лоток)
- tpl-phase-filter-complete, Show Complete → «Показать итог»
- paste-same-place / paste-current-level → «С выравниванием по тому же месту / по текущему уровню»

## Длинные надписи
Не сокращены (глоссарий или нет короче синонима): ribbon-mep («Инженерные системы» — по глоссарию), tool-footing, foundation-pad («Фундамент под колонну» — по глоссарию), prop-cover («Защитный слой»), action-group/action-ungroup, tool-join-roof, category-conduit, linepattern-dash-dot, vg-by-default, vg-cut-fill, vg-move-down, vg-add-rule, ph-later, action-rebar-sides, dlg-mep-sizing, tpl-phase-filter-new. Остальные предупреждения — имена типов, `tx-*` и значения в таблицах, не кнопки ленты.

## Прочее
Глоссарий не нарушал; несогласий нет. Сборку и тесты Rust не запускал (по заданию). Временные файлы — `.tmp/` в worktree (скрипты сверки и черновики блоков), в git не добавлены.
Уборка: удалено 0, 0 байт; оставлено `.tmp/` (~100 КБ, черновики) — удалить координатору вместе с worktree.

## Часть 3

# Отчёт: перевод части 3 (en.ftl 2001–3031, Recovery … Design options)

- Файл: `ru-work/parts/ru.3.ftl`. `python -I ru-work/check_part.py 3`: строк 1031/1031, ошибок 0,
  предупреждений 31 (HEAD 46a24c3e; в report-коммите меняется только REPORT.md). База `wendamys/ru-locale`
  d4f42eb5 слита (Already up to date), проверка повторена после merge.
- Предупреждения: латиница — только форматы/программы/клавиши (BMP, GIF, WebP, OpenStreetMap, Overpass,
  Delete, Escape, CAD, CraftHouse); остальные — длина надписей (см. ниже). Самопроверка: 100 случайных строк.

## Новые термины
- Mass / Massing / Mass floor → Масса / Массы / Этаж массы; Loft (mass) → Переход между контурами (масса-переход)
- Push/Pull → Сдвиг грани; Extrusion / Revolution / Sweep / Blend (формы компонента) → Выдавливание / Вращение / Сдвиг / Переход
- Sweep (элемент: плинтусы, молдинги) / Swept profile → Погонаж / погонажный элемент
- Terrain, Topography → Рельеф; Building pad → Площадка здания; Cut/Fill slope → Откос выемки/насыпи; Contour lines → Горизонтали
- Callout → Выносной элемент; Detail view → вид узла; имя по умолчанию «Узел { $n }»
- Design option / Option set / Primary / Main model → Вариант проекта / Набор вариантов / Основной / Основная модель
- Graph / Node / Wire / List lacing → Граф / Узел / Связь / Сопоставление списков
- Options bar → Панель параметров; Object snaps → Объектные привязки; Explode → Расчленить
- Level mark / FFL / SSL → Отметка уровня / (чист.) / (черн.); Dimension mark → Засечка; Witness line → Выносная линия
- Winders / Stringers / Waist → Забежные ступени / Косоуры / Толщина плиты марша; Ramp / Flight → Пандус / Марш
- Line styles Thin/Medium/Wide/Hidden/Dashed/Center/Dotted → Тонкая/Средняя/Толстая/Невидимая/Штриховая/Осевая/Пунктирная
- Development copy (плагина) → Рабочая копия; Plugin index → Каталог плагинов

## Сомнительные строки
- `stair-issue-blondel` и др. «Blondel 2R + T» → «Блондель 2h + b»: заменил обозначения на русские (h — подступёнок, b — проступь).
- `lib-source-online` «3D CraftHouse» → «3D CraftHouse (онлайн)»: имя без изменений check_part считает непереведённым.
- `default-callout-name` «Detail { $n }» → «Узел { $n }»: по глоссарию «Узел» — Connection; для вида узла по СПДС естественно, но возможна путаница.
- `tpl-level-mark-finished/raw` «(FFL)/(SSL)» → «(чист.)/(черн.)»: общепринятой короткой русской аббревиатуры нет.
- `stair-landing-beams-tee` «T» → «Т» (кириллица); `stair-landing-beams-cross` → «Крестом».
- `tb-field-sheet-name/project-name` → «Имя листа/проекта» по глоссарию (по СПДС привычнее «Наименование»).
- `plug-by` «by { $authors }» → «Авторы: { $authors }»: при одном авторе звучит неточно.
- `plug-when-view` → «Откройте вид: { $kind }» ($kind — имя категории вида).
- `category-sweep` «Sweeps» → «Погонаж» (собирательное, для краткости).
- `prop-date-hint`, `msg-invalid-date`: YYYY-MM-DD → ГГГГ-ММ-ДД (только подсказка; формат ввода не меняется).

## Длинные надписи (не сократил)
`tool-callout`, `section-callout`, `callout-remove`, `tx-create/remove-callout` («Выносной элемент»),
`prop-stair-winders`, `stair-post-inner`, `pref-panel-layout`, `panel-reset-layout`, `opt-sweep-flip`
(«Сменить сторону» — термин глоссария), `graph-lacing`, `action-turn-90`, `osm-layer-railways`,
`lib-source-online`; остальные предупреждения — строки окон/свойств/истории (tx-), где места хватает.

## Осталось
Ничего по части 3. Глоссарий не менял, расхождений с ним нет.

## Часть 4

# Часть 4 русского перевода — отчёт

- `python -I ru-work/check_part.py 4`: строк 1231/1231, ошибок 0, предупреждений 91 (37 — латиница: стандарты ABNT/IPE/HEA/SHS, коды команд и значений в «ёлочках», OpenEXR/Radiance, Ctrl+Plus, ложные срабатывания на `{ $total }`/`{ $month }`; 54 — длина). HEAD 85ced5a7; база `wendamys/ru-locale` d4f42eb5 (merge — «Already up to date», только локальная ветка, в origin её нет).
- Сделано: `ru-work/parts/ru.4.ftl` (en.ftl 3032–4262) и `examples/plugins/quick_rooms/locales/ru.toml` (34 ключа, совпадают с en.toml, проверено tomllib). Самопроверка — 100 случайных строк (seed 4) перечитаны, правка одной строки.

## Новые термины
- Lightning protection / air terminal / down conductor / earth electrode / earth ring → Молниезащита / Молниеприёмник / Токоотвод / Заземлитель / Контур заземления; rolling sphere → катящаяся сфера
- Structured cabling / data outlet / rack / permanent link → Кабельная сеть (СКС) / Розетка СКС / Шкаф / постоянная линия
- Pipe series → Серия труб; socket (fitting) → Раструб; CPVC/LDPE/PVC-U → ХПВХ/ПЭВД/НПВХ
- Fill pattern kind Drafting/Model → Чертёжная/Модельная; line family (в .pat) → группа линий; dashes → штрихи
- Profile Manager → Диспетчер профилей; I-section/T-section/channel/angle/hat → Двутавр/Тавр/Швеллер/Уголок/Шляпный профиль; lip → отгиб; section modulus → момент сопротивления; radius of gyration → радиус инерции
- Steel building → Стальное здание; bay → шаг (рам); purlins/girts → прогоны/ригели фахверка; rafter (рамы) → ригель рамы
- Truss chord/web/post → пояс/раскос/стойка; King/Queen post → Одностоечная/Двухстоечная
- Light Steel Frame → Каркас из ЛСТК; stud/track/plate/header/blocking → стойка/направляющая/обвязка/перемычка/распорки
- Base/End/Fin plate, Hanger → Опорная плита / Фланец / Рёберная пластина / Башмак
- Pile cap / tie beams / cut-off / toe → Ростверк / связевые балки / отметка срезки / отметка острия
- Phase (группировка обозревателя) → Стадия; Discipline → Раздел; Browser organization → Организация обозревателя
- Render / samples / bounces / clay / tone map → Визуализировать / Выборки / Отражения / Глина / Тональное отображение; Preset → Предустановка
- View mark → Обозначение вида; grid bubble / mark head → кружок; Quick Access Toolbar → Панель быстрого доступа
- Elevation profile (стены) → профиль по фасаду; spline wall → сплайновая стена; Fillet Arc → Сопряжение дугой

## Сомнительные строки
- `tpl-param-bottom-rail` BottomRail → «НижняяОбвязка»: имя параметра в одно слово, как у соседних tpl-param (стиль частей 1–3 не видел).
- `browser-group-phase` Phase → «Стадия»: сверить с переводом раздела Phases в другой части.
- `tx-*`: глаголы в инфинитиве («Окрасить грань») по образцу uk-UA; сверить со стилем tx в частях 1–3.
- `action-render` Render → «Визуализировать»: одна и та же строка — кнопка ленты и главная кнопка окна.
- `sun-date-pattern` → `{ $day }.{ $month }.{ $year }` (формат даты на траектории солнца).
- `msg-sun-date`/`msg-sun-time`: YYYY-MM-DD/HH:MM → ГГГГ-ММ-ДД/ЧЧ:ММ (ввод по-прежнему в формате ISO).
- `msg-sun-*-unknown`, `macro-check-*`: коды (still, single_day, «command», «$step») оставлены латиницей в «ёлочках» вместо прямых кавычек.
- `ribbon-place-tool` Place { $tool } → «Размещение: { $tool }» (в заголовке «Изменить | Размещение: Стена»).
- `cmd-desc-wall-tool-pick-lines`: «Chain» → «Цепочка» — сверить с `sketch-chain` из части 2.
- `pipe-series-eu-pvcu-1329/1401`: добавлено «внутренняя»/«наружная» канализация по назначению стандартов EN 1329/EN 1401, чтобы серии различались.
- `connection-end-plate` End Plate → «Фланец» (сокращено с «Торцевой фланец»).

## Длинные надписи (не сократились без потери смысла)
action-render, render-save, render-tone-map, action-earth-ring, prop-girts, framing-wood, action-wood-frame, connection-fin-plate, schedule-sort-add, macro-ed-run-end, macro-ed-restart, macro-ed-format, pform-run-again, category-view-mark, tag-types-marks, level-heads, mat-texture-load. Остальные предупреждения о длине — имена типов, серий, предустановок и записи истории отмены (tx-*), их ширина не ограничена.

## Осталось
Ничего. Уборка: удалена `.tmp/` (c1–c6.ftl, ck.py, 83 707 байт), оставлено 0. Сборка и тесты Rust не запускались (по заданию — координатор после сшивки).

