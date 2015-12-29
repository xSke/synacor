// Modified version of https://github.com/glguy/synacor-vm/blob/master/teleport.c
// I don't know C, and every other language I've tried has ran out of stack space quickly
// Googled around and found this implementation of the modified ackermann (or whatever it's called)
// Thanks @glguy!

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

static uint16_t cache[5][0x8000];
static uint16_t h; /*register 8*/

static inline void reset_cache(void) {
  memset(cache, 0xff, sizeof(cache));
}

/* Modified Ackermann function */
static uint16_t top(uint16_t a, uint16_t b) {

  if (cache[a][b] != 0xffff) return cache[a][b];
  if (a == 0)                return (b+1) & 0x7fff;
  if (b == 0)                return top(a-1,h);

  return top(a-1, cache[a][b-1] = top(a,b-1));
}

int main (int argc, char *argv[]) {

   for (h = 0x5000; h < 0x8000; h++) {

        reset_cache();

        uint16_t t = top(4,1);
        printf("%d: %d\r\n", h, t);
        fflush(stdout);

        if (t == 6) {
          printf("FOUND IT: %d\r\n", h);
          fflush(stdout);
          return h;
        }
   }

   printf("Search failed\n");
   fflush(stdout);
   return 1;
}
