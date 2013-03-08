#include <stdio.h>
#include <termios.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>

struct termios orig;

void unbuffer() {
  struct termios new;
  tcgetattr(0, &orig);
  memcpy(&new, &orig, sizeof(struct termios));
  new.c_lflag &= ~(ICANON | ECHO);
  new.c_cc[VTIME] = 0;
  new.c_cc[VMIN] = 1;
  tcsetattr(0, TCSANOW, &new);
}

void restore() {
  tcsetattr(0, TCSANOW, &orig);
}

int getbyte() {
  char c = 0;
  int r = read(0, &c, 1);
  if (r!=1) {
    return -1;
  }
  return c;
}

#ifdef TEST
int main() {
  unbuffer();
  while (1) {
    int c = getbyte();
    if(c < 0 || c == 4) {
      break;
    } else {
      printf(" char read: %x\n", c);
    }
  }
  restore();
  return 0;
}
#endif
