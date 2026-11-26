# Части перевода (база — `assets/i18n/en.ftl` на коммите-основе ветки)

| Часть | Строки en.ftl | Разделы (заголовки `#`) | Файл результата |
|---|---|---|---|
| 1 | 1–1160 | Categories … Materials | `ru-work/parts/ru.1.ftl` |
| 2 | 1161–2000 | Modify tools … Building services (MEP) | `ru-work/parts/ru.2.ftl` |
| 3 | 2001–3031 | Recovery … Detail components | `ru-work/parts/ru.3.ftl` |
| 4 | 3032–4262 | Material textures … Schedule properties (до конца) + `examples/plugins/quick_rooms/locales/ru.toml` | `ru-work/parts/ru.4.ftl`, `examples/plugins/quick_rooms/locales/ru.toml` |

Файл части — те же строки en.ftl в том же порядке и количестве: строки-комментарии и пустые строки
копируются как есть, в строках `ключ = значение` переводится только значение.
Проверка: `python -I ru-work/check_part.py <номер части>`.
