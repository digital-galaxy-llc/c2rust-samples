//----------------------------------------------------------------
// funky.c
// by Eli Bendersky (eliben@gmail.com)
//
// This code is in the public domain.
// Original source : https://github.com/eliben/pycparser/blob/main/examples/c_files/funky.c
//----------------------------------------------------------------


char foo(void)
{
    return '1';
}

int maxout_in(int paste, char** matrix)
{
    char o = foo();
    return (int) matrix[1][2] * 5 - paste;
}

int main()
{
    auto char* multi = "a multi";
}
