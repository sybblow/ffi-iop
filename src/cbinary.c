#include <stdio.h>

const char* rust_text();

unsigned char three[3];


unsigned int cmax(unsigned int num1, unsigned int num2)
{
   unsigned int result;

   if (num1 > num2)
      result = num1;
   else
      result = num2;

   return result;
}

int main(void) {
   const char* text = rust_text();

   unsigned char somewhere = three[0];

   printf("%s - %d\n", text, somewhere);

   return 0;
}
