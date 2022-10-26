## Mac kernel

Two ways the user layer controls kernel extension module:

- Character driven
- Socket

## Mac kernel control and event

Socket in MacOs supports domain PF_SYSTEM that has two protocals
- SYSPROTO_CONTROL
- SYSPROTO_EVENT

## Register kernel control

``` c
struct kern_ctl_reg {
    ctl_name,
    ctl_id,
    ctl_unit,
    ctl_flags,
}
```

## Code
``` c
errno_t error;

struct kern_ctl_reg ep_ctl; // Initialize control

kern_ctl_ref kctlref;

bzero(&ep_ctl, sizeof(ep_ctl)); // sets ctl_unit to 0

ep_ctl.ctl_id = 0; /* OLD STYLE: ep_ctl.ctl_id = kEPCommID; */

ep_ctl.ctl_unit = 0;

strcpy(ep_ctl.ctl_name, "org.mklinux.nke.foo");

ep_ctl.ctl_flags = CTL_FLAG_PRIVILEGED & CTL_FLAG_REG_ID_UNIT;

ep_ctl.ctl_send = EPHandleWrite;

ep_ctl.ctl_getopt = EPHandleGet;

ep_ctl.ctl_setopt = EPHandleSet;

ep_ctl.ctl_connect = EPHandleConnect;

ep_ctl.ctl_disconnect = EPHandleDisconnect;

error = ctl_register(&ep_ctl, &kctlref);

/* A simple setsockopt handler */

errno_t EPHandleSet( kern_ctl_ref ctlref, unsigned int unit, void *userdata, int opt, void *data, size_t len ) {

    int error = EINVAL;

    #if DO_LOG

    log(LOG_ERR, "EPHandleSet opt is %d\n", opt);

    #endif

    switch ( opt )

    {

    case kEPCommand1: // program defined symbol

    error = Do_First_Thing();

    break;

    case kEPCommand2: // program defined symbol

    error = Do_Command2();

    break;

    }

    return error;
}

/* A simple A simple getsockopt handler */

errno_t EPHandleGet(kern_ctl_ref ctlref, unsigned int unit, void *userdata, int opt, void *data, size_t *len) {

    int error = EINVAL;

    #if DO_LOG

    log(LOG_ERR, "EPHandleGet opt is %d *****************\n", opt);

    #endif

    return error;

}

/* A minimalist connect handler */

errno_t EPHandleConnect(kern_ctl_ref ctlref, struct sockaddr_ctl *sac, void **unitinfo) {

    #if DO_LOG

    log(LOG_ERR, "EPHandleConnect called\n");

    #endif

    return (0);

}

/* A minimalist disconnect handler */

errno_t EPHandleDisconnect(kern_ctl_ref ctlref, unsigned int unit, void *unitinfo) {

    #if DO_LOG

    log(LOG_ERR, "EPHandleDisconnect called\n");

    #endif

    return;

}

/* A minimalist write handler */

errno_t EPHandleWrite(kern_ctl_ref ctlref, unsigned int unit, void *userdata, mbuf_t m, int flags) {
    #if DO_LOG

    log(LOG_ERR, "EPHandleWrite called\n");

    #endif

    return (0);
}
```

5.客户端代码片段

``` c
struct sockaddr_ctl addr;
int ret = 1;
fd = socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL);
if (fd != -1) {
    bzero(&addr, sizeof(addr)); // sets the sc_unit field to 0
    addr.sc_len = sizeof(addr);
    addr.sc_family = AF_SYSTEM;
    addr.ss_sysaddr = AF_SYS_CONTROL;
    #ifdef STATIC_ID
        addr.sc_id = kEPCommID; // should be unique - use a registered Creator ID here
        addr.sc_unit = kEPCommUnit; // should be unique.
    #else
    {
        struct ctl_info info;
        memset(&info, 0, sizeof(info));
        strncpy(info.ctl_name, MYCONTROLNAME, sizeof(info.ctl_name));
        if (ioctl(fd, CTLIOCGINFO, &info)) {
            perror("Could not get ID for kernel control.\n");
            exit(-1);
        }
        addr.sc_id = info.ctl_id;
        addr.sc_unit = 0;
    }

    #endif
    result = connect(fd, (struct sockaddr *)&addr, sizeof(addr));
    if (result) {
        fprintf(stderr, "connect failed %d\n", result);
    }
} else { /* no fd */
    fprintf(stderr, "failed to open socket\n");
}

if (!result) {
    result = setsockopt( fd, SYSPROTO_CONTROL, kEPCommand1, NULL, 0);
    if (result){
        fprintf(stderr, "setsockopt failed on kEPCommand1 call - result was %d\n", result);
    }
}

```
## References
[mac kernal control](https://blog.csdn.net/weixin_30125993/article/details/116686616)
[utun](https://github.com/mafintosh/utun/blob/master/utun.cc)
[open vpn](https://github.com/OpenVPN/openvpn3/blob/master/openvpn/tun/mac/utun.hpp)