#include <stdio.h>

const char* rust_text();

int main(void) {
   const char* text = rust_text();

   printf(text);

   return 0;
}
