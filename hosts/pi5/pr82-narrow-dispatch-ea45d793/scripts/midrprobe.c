#include <stdio.h>
#include <sys/auxv.h>
int main(void){
  unsigned long hw=getauxval(AT_HWCAP);
  printf("AT_HWCAP=%#lx HWCAP_CPUID(bit11)=%lu\n", hw, (hw>>11)&1);
  if((hw>>11)&1){ unsigned long m; __asm__ volatile("mrs %0, MIDR_EL1":"=r"(m)); printf("MIDR_EL1=%#lx part=%#lx\n", m, (m>>4)&0xfff); }
  else printf("skipping MIDR read (no HWCAP_CPUID)\n");
  return 0;
}
