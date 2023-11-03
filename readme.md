# rgrep

**Command**
 - rgrep {keyword} {relative path}

**Output**
 - 파일의 각 line에 keyword가 있는 경우, 해당 파일과 줄 번호를 출력한다.

**Condition**
1. relative path가 디렉토리인 경우 디렉토리 내 모든 파일에 대해 검사를 진행한다.
2. relative path 내에 또 다른 디렉토리가 존재하는 경우, 각 디렉토리 내 모든 파일에 대한 검사 또한 진행한다.
4. 동일한 파일에 대한 검사 결과는 한 번에 출력되어야 한다.
5. Directory 내 symlink는 없다고 가정한다.
6. 파일들은 모두 UTF8 인코딩으로 작성된 Text파일이라고 가정한다.
