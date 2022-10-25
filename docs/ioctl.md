## driver

``` c
// ioctl-test-driver.c
......

static const struct file_operations fops = {
    .owner = THIS_MODULE,
    .open = test_open,
    .release = test_close,
    .read = test_read,
    .write = etst_write,
    .unlocked_ioctl = test_ioctl,
};

......

static long test_ioctl(struct file *file, unsigned int cmd, \
                        unsigned long arg)
{
    //printk("[%s]\n", __func__);

    int ret;
    struct msg my_msg;

    /* 检查设备类型 */
    if (_IOC_TYPE(cmd) != IOC_MAGIC) {
        pr_err("[%s] command type [%c] error!\n", \
            __func__, _IOC_TYPE(cmd));
        return -ENOTTY; 
    }

    /* 检查序数 */
    if (_IOC_NR(cmd) > IOC_MAXNR) { 
        pr_err("[%s] command numer [%d] exceeded!\n", 
            __func__, _IOC_NR(cmd));
        return -ENOTTY;
    }    

    /* 检查访问模式 */
    if (_IOC_DIR(cmd) & _IOC_READ)
        ret= !access_ok(VERIFY_WRITE, (void __user *)arg, \
                _IOC_SIZE(cmd));
    else if (_IOC_DIR(cmd) & _IOC_WRITE)
        ret= !access_ok(VERIFY_READ, (void __user *)arg, \
                _IOC_SIZE(cmd));
    if (ret)
        return -EFAULT;

    switch(cmd) {
    /* 初始化设备 */
    case IOCINIT:
        init();
        break;

    /* 读寄存器 */
    case IOCGREG:
        ret = copy_from_user(&msg, \
            (struct msg __user *)arg, sizeof(my_msg));
        if (ret) 
            return -EFAULT;
        msg->data = read_reg(msg->addr);
        ret = copy_to_user((struct msg __user *)arg, \
                &msg, sizeof(my_msg));
        if (ret) 
            return -EFAULT;
        break;

    /* 写寄存器 */
    case IOCWREG:
        ret = copy_from_user(&msg, \
            (struct msg __user *)arg, sizeof(my_msg));
        if (ret) 
            return -EFAULT;
        write_reg(msg->addr, msg->data);
        break;

    default:
        return -ENOTTY;
    }

    return 0;
}
```
## user code

``` c
// ioctl-test.c

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/ioctl.h> 

#include "ioctl-test.h"

int main(int argc, char **argv)
{

    int fd;
    int ret;
    struct msg my_msg;

    fd = open("/dev/ioctl-test", O_RDWR);
    if (fd < 0) {
        perror("open");
        exit(-2);
    }

    /* 初始化设备 */
    ret = ioctl(fd, IOCINIT);
    if (ret) {
        perror("ioctl init:");
        exit(-3);
    }

    /* 往寄存器0x01写入数据0xef */
    memset(&my_msg, 0, sizeof(my_msg));
    my_msg.addr = 0x01;
    my_msg.data = 0xef;
    ret = ioctl(fd, IOCWREG, &my_msg);
    if (ret) {
        perror("ioctl read:");
        exit(-4);
    }

    /* 读寄存器0x01 */
    memset(&my_msg, 0, sizeof(my_msg));
    my_msg.addr = 0x01;
    ret = ioctl(fd, IOCGREG, &my_msg);
    if (ret) {
        perror("ioctl write");
        exit(-5);
    }
    printf("read: %#x\n", my_msg.data);

    return 0;
}
```