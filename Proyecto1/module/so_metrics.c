#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/proc_fs.h>      // Funciones para crear archivos en /proc
#include <linux/seq_file.h>     // Funciones para escribir en archivos en /proc
#include <linux/mm.h>           // Funciones para manejar la memoria
#include <linux/sched.h>        // Funciones para manejar procesos
#include <linux/timer.h>        // Funciones para manejar timers
#include <linux/jiffies.h>      // Macros para manejar jiffies (ticks del sistema)

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Sergio Larios");
MODULE_DESCRIPTION("Módulo para leer información de memoria y CPU");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo_202111849"

static int sysinfo_show(struct seq_file *m, void *v) {
    struct sysinfo si;      // Estructura que contiene la información de la memoria

    si_meminfo(&si);        // Obtiene la información de la memoria

    seq_printf(m, "Total RAM: %lu KB\n", si.totalram * 4);
    seq_printf(m, "Free RAM: %lu KB\n", si.freeram * 4);
    seq_printf(m, "Shared RAM: %lu KB\n", si.sharedram * 4);
    seq_printf(m, "Buffered RAM: %lu KB\n", si.bufferram * 4);
    seq_printf(m, "Total Swap: %lu KB\n", si.totalswap * 4);
    seq_printf(m, "Free Swap: %lu KB\n", si.freeswap * 4);

    seq_printf(m, "Number of processes: %d\n", num_online_cpus());

    return 0;
};

static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
};

static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read
};

static int __init sysinfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "Módulo de información del sistema cargado\n");
    return 0;
}

static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "Módulo de información del sistema descargado\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);