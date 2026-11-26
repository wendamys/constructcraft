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
