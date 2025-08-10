/* simple C program to check ioctl raw values */

#include <stdio.h>
#include <linux/ptp_clock.h>

int main() {
    printf("PTP_CLOCK_GETCAPS: 0x%08x\n", PTP_CLOCK_GETCAPS);
    printf("PTP_SYS_OFFSET: 0x%08x\n", PTP_SYS_OFFSET);
    printf("PTP_SYS_OFFSET_PRECISE: 0x%08x\n", PTP_SYS_OFFSET_PRECISE);
    printf("PTP_SYS_OFFSET_EXTENDED: 0x%08x\n", PTP_SYS_OFFSET_EXTENDED);

    return 0;
}
