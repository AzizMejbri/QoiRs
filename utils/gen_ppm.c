#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>


#define WIDTH 100
#define HEIGHT 100
#define PIXEL_SIZE 4

int 
main(){
  srand(time(NULL));
  int ppm = open("../test/file.ppm", O_CREAT | O_WRONLY | O_TRUNC, 0644);
  char buffer[WIDTH*HEIGHT*PIXEL_SIZE];
  char header[] = { 'P', '6', 0xA, '1', '0', '0', ' ', '1', '0', '0', 0xa, '2', '5', '5', 0xA  };
  memcpy(buffer, header, 15);
  for(unsigned i = 15; i < WIDTH * HEIGHT * PIXEL_SIZE; i++){
    buffer[i] = rand()%255;
  }
  write(ppm, buffer, WIDTH*HEIGHT*PIXEL_SIZE);
  close(ppm);
  printf("\e[34mWritten random Data to file.ppm!\e[m\n");
  return 0;
}
