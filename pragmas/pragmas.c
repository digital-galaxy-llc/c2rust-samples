//----------------------------------------------------------------
// pragmas.c
// by Eli Bendersky (eliben@gmail.com)
//
// This code is in the public domain.
// Original source : https://github.com/eliben/pycparser/blob/main/examples/c_files/pragmas.c
//----------------------------------------------------------------

int i;

#pragma joe

int main() {
  int a = 1;
#pragma inmain

  for (int i = 0; i < 3; i++) {
#pragma infor
    a += i;
  }

  return a;
}
