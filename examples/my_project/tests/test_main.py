from pysnaptest import snapshot
from my_project.main import main


@snapshot()
def test_main():
    return main()
