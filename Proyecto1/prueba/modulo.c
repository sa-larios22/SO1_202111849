#include <linux/module.h>
#include <linux/kernel.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Sergio");
MODULE_DESCRIPTION("Modulo básico");

static int __init mymodule_init(void) {
    printk(KERN_INFO "Módulo de Kernel cargado correctamente\n");
    return 0;
}

static void __exit mymodule_exit(void) {
    printk(KERN_INFO "Módulo de Kernel descargado correctamente\n");
}

module_init(mymodule_init);
module_exit(mymodule_exit);