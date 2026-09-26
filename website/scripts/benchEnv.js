#!/usr/bin/env -S bun

/**
 * 获取当前测试运行环境信息（优先读取 results.json 中的 sysinfo 数据，降级使用 node:os 标准模块）
 * 严格遵循 .agents/skills/js_review 规范
 */

import os from "node:os";

const envFromNodeOs = () => {
  const os_type = os.type(),
    os_rel = os.release(),
    arch = os.arch(),
    cpus = os.cpus() || [],
    cores = cpus.length || 1,
    cpu_brand = cpus[0]?.model?.trim() || cores + " vCPU",
    ram_gb = Math.round(os.totalmem() / (1024 * 1024 * 1024)),
    os_str = os_type + " " + os_rel,
    summary_zh =
      os_str + " (" + arch + ") · " + cpu_brand + " (" + cores + "核) · " + ram_gb + "GB 内存",
    summary_en =
      os_str + " (" + arch + ") · " + cpu_brand + " (" + cores + " Cores) · " + ram_gb + " GB RAM";

  return {
    os: os_str,
    arch,
    cpu: cpu_brand,
    cores,
    ram_gb,
    summary: summary_zh,
    summary_zh,
    summary_en,
  };
};

export const benchEnvGet = (existing_env) => {
  if (existing_env && existing_env.os) {
    const os_str = existing_env.os,
      arch = existing_env.arch || os.arch(),
      cpu_str = existing_env.cpu || (existing_env.cores ? existing_env.cores + " vCPU" : "CPU"),
      cores = existing_env.cores || 1,
      ram_gb = existing_env.ram_gb || 0,
      summary_zh =
        existing_env.summary ||
        os_str +
          " (" +
          arch +
          ") · " +
          cpu_str +
          " (" +
          cores +
          "核)" +
          (ram_gb > 0 ? " · " + ram_gb + "GB 内存" : ""),
      summary_en =
        os_str +
        " (" +
        arch +
        ") · " +
        cpu_str +
        " (" +
        cores +
        " Cores)" +
        (ram_gb > 0 ? " · " + ram_gb + " GB RAM" : "");

    return {
      os: os_str,
      arch,
      cpu: cpu_str,
      cores,
      ram_gb,
      summary: summary_zh,
      summary_zh,
      summary_en,
    };
  }
  return envFromNodeOs();
};
