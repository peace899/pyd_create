import sys
from pathlib import Path

current_path = Path(__file__).parent.resolve()
lib_path = current_path / "extlib"
print(lib_path)
sys.path.append(str(lib_path))
import my_extension


print(my_extension.add(1, 2))