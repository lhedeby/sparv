import subprocess
import re
import itertools
import json
from os import listdir



def main():
    # build()
    interpreter = "C:/Users/ludwi/repos/krax/src/bin/Debug/net8.0/krax.exe"
    test = listdir("./test")
    err_tests = 0
    ok_tests = 0
    for file_name in test:
        print(f"running file {file_name}")
        ok = test_file(interpreter, f"./test/{file_name}")
        if ok:
            ok_tests += 1
        else:
            err_tests += 1

    print("\n___Test results___")
    print_ok(f"Ok: {ok_tests}")
    if err_tests:
        print_err(f"Err: {err_tests}")
        print_err("Error(s) occured in tests!")
    else:
        print(f"Err: {err_tests}")
        print_ok("All tests Ok!")

def build() -> str:
    """
        Builds the rust interpreter and returns the the path
        to the executable.

        Args:
            None

        Returns:
            str: The path to the executable
    """
    print("Running dotnet build!")
    output = subprocess.run(["dotnet", "build"], shell=True, capture_output=True)

    # lines = output.stdout.decode('utf-8').splitlines()
    # jsons = [json.loads(x) for x in lines]
    #
    # build_success = jsons[-1]['success']
    # if not build_success:
    #     print_err("Cargo build ERR")
    #     print(lines)
    #     exit(1)
    # interpreter_path = jsons[-2]['executable']
    # print_ok("Cargo build OK");
    # return interpreter_path
    return ""


def test_file(interpreter: str, path: str):
    """
        Find all the '// expected:' comments in the sparv code
        and test if the actual output of the program is what
        we expect.

        Args:
            interpreter (str): Path to the executable
            path (str): Path to the .sparv file to run
        
        Returns:
            bool: Returns True if all the tests in the file
                  passes. Otherwise returns False.

    """
    file = open(path, mode="r")
    content = file.read()
    expected = [
        f"Running: {path}" ,
        *re.findall("(?<=// expected: ).*", content),
        "successfully ran program"
    ]
    print(interpreter)

    output = subprocess.run([interpreter, path], shell=True, capture_output=True)
    actual = output.stdout.decode('utf-8').splitlines()

    errors = [
        (exp, act)
        for exp, act
        in itertools.zip_longest(expected, actual, fillvalue=None)
        if exp != act
    ]
    errors += output.stderr.decode('utf-8').splitlines()

    if errors:
        print_err(f"\nError in test(s): '{path}'")
    for error in errors:
        print_err(f"Expected: {error[0]}, Actual: {error[1]}")
    return not errors

def print_ok(text):
    """ 
        Print green text.

        Args:
            text (str): The text to print.

        Returns:
            None
    """
    print(f"\033[92m{text}\033[0m")
def print_err(text):
    """ 
        Print red text.

        Args:
            text (str): The text to print.

        Returns:
            None
    """
    print(f"\033[91m{text}\033[0m")

if __name__ == "__main__": main()
