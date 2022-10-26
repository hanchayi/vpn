
## protocal

dir    2bit
size   14bit
type   8bit
nr     8bit

### dir
(direction)，ioctl命令访问模式（数据传输方向），占据2bit，可以为_IOC_NONE、_IOC_READ、_IOC_WRITE、_IOC_READ | _IOC_WRITE，分别指示了四种访问模式：无数据、读数据、写数据、读写数据；
### type
(device type)，设备类型，占据8bit，在一些文献中翻译为“幻数”或者“魔数”，可以为任意char型字符，例如‘a’、‘b’、‘c’等等，其主要作用是使ioctl命令有唯一的设备标识。tips：Documentions/ioctl-number.txt记录了在内核中已经使用的“魔数”字符，为避免冲突，在自定义ioctl命令之前应该先查阅该文档。



### nr
(number)，命令编号/序数，占据8bit，可以为任意unsigned char型数据，取值范围0~255，如果定义了多个ioctl命令，通常从0开始编号递增；

### size
涉及到ioctl第三个参数arg，占据13bit或者14bit（体系相关，arm架构一般为14位），指定了arg的数据类型及长度，如果在驱动的ioctl实现中不检查，通常可以忽略该参数。

```c
// include/uapi/asm-generic/ioctl.h

#define _IOC(dir,type,nr,size) \
    (((dir)  << _IOC_DIRSHIFT) | \
     ((type) << _IOC_TYPESHIFT) | \
     ((nr)   << _IOC_NRSHIFT) | \
     ((size) << _IOC_SIZESHIFT))
```




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

## References

- https://zhuanlan.zhihu.com/p/478247461
- https://baike.baidu.com/item/ioctl/6392403?fr=aladdin
