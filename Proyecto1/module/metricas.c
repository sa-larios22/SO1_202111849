#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/proc_fs.h>      // Para crear archivos en /proc
#include <linux/seq_file.h>     // Para escribir en archivos en /proc
#include <linux/mm.h>           // Para manejar memoria
#include <linux/sched.h>        // Para manejar procesos
#include <linux/timer.h>        // Para manejar timers
#include <linux/jiffies.h>      // Para manejar jiffies (ticks del sistema)
#include <linux/slab.h>         // Para asignación de memoria dinámica
#include <linux/fs.h>           // Para leer archivos
#include <linux/uaccess.h>      // Para copiar datos entre user-space y kernel-space
#include <linux/delay.h>        // Para `msleep()`

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Sergio Larios");
MODULE_DESCRIPTION("Módulo para capturar métricas de memoria y contenedores en /proc");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo_202111849"
#define CPU_STAT_FILE "/proc/stat"

static unsigned long prev_active = 0, prev_total = 0;

static unsigned int get_cpu_usage(void) {
    struct file *f;
    char buf[256];
    loff_t pos = 0;
    ssize_t bytes_read;
    unsigned long user, nice, system, idle, iowait, irq, softirq, steal;
    unsigned long active, total;
    unsigned int usage = 0;

    f = filp_open(CPU_STAT_FILE, O_RDONLY, 0);
    if (IS_ERR(f)) {
        return 0;
    }

    bytes_read = kernel_read(f, buf, sizeof(buf) - 1, &pos);
    filp_close(f, NULL);

    if (bytes_read <= 0) {
        return 0;
    }

    buf[bytes_read] = '\0';

    if (sscanf(buf, "cpu %lu %lu %lu %lu %lu %lu %lu %lu", &user, &nice, &system, &idle, &iowait, &irq, &softirq, &steal) < 8) {
        return 0;
    }

    active = user + nice + system + irq + softirq + steal;
    total = active + idle + iowait;

    if (prev_total > 0) {
        usage = ((active - prev_active) * 100) / (total - prev_total);
    }

    prev_active = active;
    prev_total = total;

    return usage;
}

static int sysinfo_show(struct seq_file *m, void *v) {
    struct sysinfo si;
    struct task_struct *task;
    unsigned long totalram, freeram, usedram;

    si_meminfo(&si);
    totalram = si.totalram * si.mem_unit / 1024;  // Convertir a KB
    freeram = si.freeram * si.mem_unit / 1024;    // Convertir a KB
    usedram = totalram - freeram;

    seq_printf(m, "{\n");
    seq_printf(m, "  \"Memoria\": {\n");
    seq_printf(m, "    \"Total_KB\": %lu,\n", totalram);
    seq_printf(m, "    \"Libre_KB\": %lu,\n", freeram);
    seq_printf(m, "    \"En_uso_KB\": %lu\n", usedram);
    seq_printf(m, "  },\n");

    unsigned int cpu_usage = get_cpu_usage();  // Obtiene el uso actual de CPU

    seq_printf(m, "  \"CPU\": {\n");
    seq_printf(m, "    \"Uso_Porcentaje\": %u\n", cpu_usage);
    seq_printf(m, "  },\n");
    

    seq_printf(m, "  \"Contenedores\": [\n");

    int first = 1;  // Para controlar la coma en JSON
    for_each_process(task) {
        cond_resched();  // Evita soft lockups

        if (strstr(task->comm, "docker") || strstr(task->comm, "container") || strstr(task->comm, "stress")) {
            struct mm_struct *mm;
            unsigned long rss = 0;

            mm = get_task_mm(task);
            if (mm) {
                rss = get_mm_rss(mm) << PAGE_SHIFT;
                mmput(mm);
            }

            if (!first) {
                seq_printf(m, ",\n");  // Solo agrega coma si no es el primer elemento
            }
            first = 0;

            seq_printf(m, "    {\n");
            seq_printf(m, "      \"PID\": %d,\n", task->pid);
            seq_printf(m, "      \"Nombre\": \"%s\",\n", task->comm);
            seq_printf(m, "      \"Cmd\": \"%s\",\n", task->comm);
            seq_printf(m, "      \"Memoria_KB\": %lu\n", rss / 1024);
            seq_printf(m, "    }");
        }
    }

    if (first) {
        seq_printf(m, "    \"No se encontraron contenedores en ejecución\"\n");
    }

    seq_printf(m, "\n  ]\n");
    seq_printf(m, "}\n");

    return 0;
}

static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}

static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
    .proc_lseek = seq_lseek,
    .proc_release = single_release
};

static int __init sysinfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "Módulo %s cargado en /proc\n", PROC_NAME);
    return 0;
}

static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "Módulo %s eliminado de /proc\n", PROC_NAME);
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);
