"""Проверка части русского перевода против en.ftl.

python -I ru-work/check_part.py 1|2|3|4   — часть ru-work/parts/ru.N.ftl
python -I ru-work/check_part.py all       — весь assets/i18n/ru.ftl

Ошибки (exit 1): число и порядок строк, ключи, подстановки { $x }, пустые значения,
непереведённые значения. Предупреждения (exit 0): латиница вне списка допустимых,
длина коротких надписей, прямые кавычки.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
PARTS = {"1": (1, 1160), "2": (1161, 2000), "3": (2001, 3031), "4": (3032, None)}
PH = re.compile(r"\{ \$[A-Za-z0-9_-]+ \}")
LATIN = re.compile(r"[A-Za-z][A-Za-z'’-]{2,}")
ALLOWED = {
    w.lower()
    for w in """ConstructCraft ConstructCAD IFC IFC2X3 IFC4 PDF SVG DXF DWG CSV glTF GLB OBJ PNG JPG JPEG
    SketchUp Revit Lua LuaLS MCP GPL GPL-3.0-or-later Draco KHR URL HTTP HTTPS API JSON TOML YAML XML UTF GitHub
    Discord Ctrl Shift Alt Enter Esc Tab Del Space Backspace PgUp PgDn IBM Plex Mono Sans Inter Noto ISO NBR ASTM
    DIN BIM CAD RGB HSV sRGB GlobalId Pset Qto MEP DCO SHA meshopt Windows macOS Linux CJK Fluent Blender
    LuaJIT Cargo Rust WGSL wgpu GPU CPU OpenGL Vulkan DirectX Metal BCF COBie IfcOpenShell FreeCAD LibreCAD
    QCAD AutoCAD ArchiCAD Rhino Dynamo Grasshopper Python stdout stderr ccad""".split()
}


def parse(line):
    s = line.strip()
    if not s or s.startswith("#"):
        return None
    if "=" not in s:
        return ("?", s)
    k, v = s.split("=", 1)
    return (k.strip(), v.strip())


def code_like(tok_ctx):
    return "`" in tok_ctx or "_" in tok_ctx or "." in tok_ctx or "%" in tok_ctx


def main():
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
        sys.stderr.reconfigure(encoding="utf-8", errors="replace")
    arg = sys.argv[1] if len(sys.argv) > 1 else ""
    en_all = (ROOT / "assets/i18n/en.ftl").read_text(encoding="utf-8").splitlines()
    if arg == "all":
        en, ru_path, first = en_all, ROOT / "assets/i18n/ru.ftl", 1
    elif arg in PARTS:
        a, b = PARTS[arg]
        en, ru_path, first = en_all[a - 1 : b], ROOT / f"ru-work/parts/ru.{arg}.ftl", a
    else:
        sys.exit("usage: check_part.py 1|2|3|4|all")
    if not ru_path.exists():
        sys.exit(f"нет файла {ru_path}")
    ru = ru_path.read_text(encoding="utf-8-sig").splitlines()
    errors, warns = [], []
    if len(ru) != len(en):
        errors.append(f"строк {len(ru)}, а в en.ftl {len(en)} — сверка по строкам до первого сдвига")
    for i, (e, r) in enumerate(zip(en, ru)):
        n = first + i
        pe, pr = parse(e), parse(r)
        if pe is None:
            if pr is not None:
                errors.append(f"{n}: в en комментарий/пустая строка, в ru — сообщение: {r[:60]}")
            continue
        if pr is None:
            errors.append(f"{n}: нет сообщения {pe[0]}")
            continue
        ke, ve = pe
        kr, vr = pr
        if ke != kr:
            errors.append(f"{n}: ключ {kr!r} вместо {ke!r}")
            continue
        if not vr:
            errors.append(f"{n} {ke}: пустое значение")
            continue
        if sorted(PH.findall(ve)) != sorted(PH.findall(vr)):
            errors.append(f"{n} {ke}: подстановки {PH.findall(vr)} вместо {PH.findall(ve)}")
        if re.search(r"\{\s*\$", vr) and len(re.findall(r"\{\s*\$[^}]*\}", vr)) != len(PH.findall(vr)):
            errors.append(f"{n} {ke}: подстановка записана не как {{ $имя }}")
        words = [w for w in LATIN.findall(ve) if w.lower() not in ALLOWED]
        if vr == ve and words and not code_like(ve):
            errors.append(f"{n} {ke}: не переведено: {vr[:70]}")
            continue
        plain = re.sub(r"`[^`]*`|\{ \$[^}]*\}|%[a-z0-9_.-]+%|\S*[._/\\]\S*", " ", vr)
        left = sorted({w for w in LATIN.findall(plain) if w.lower() not in ALLOWED})
        if left:
            warns.append(f"{n} {ke}: латиница {left}")
        if len(ve) <= 20 and len(vr) > max(12, int(len(ve) * 1.6)):
            warns.append(f"{n} {ke}: длинно для короткой надписи ({len(ve)}→{len(vr)}): {vr}")
        if '"' in vr and '"' not in ve:
            warns.append(f"{n} {ke}: прямые кавычки — нужны «ёлочки»")
    print(f"{ru_path.relative_to(ROOT)}: строк {len(ru)}/{len(en)}, ошибок {len(errors)}, предупреждений {len(warns)}")
    for x in errors[:200]:
        print("ОШИБКА", x)
    for x in warns[:300]:
        print("предупр.", x)
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
