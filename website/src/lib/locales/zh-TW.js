export default {
  "meta.title": "ulua — 用 Rust 重寫 Luau",
  "meta.description":
    "用人工智慧徹底重寫 Luau，完全按 Rust 風格程式設計。在瀏覽器沙盒中即時執行與靜態型別檢查。",
  "nav.playground": "線上體驗",
  "nav.benchmark": "效能基準",
  "nav.about": "關於重寫",
  "nav.features": "編譯为 WASM",
  "nav.lua_syntax": "Lua 5.1 語法",
  "nav.luau_syntax": "Luau 擴充",
  "nav.embed": "Rust 嵌入",
  "nav.checker": "型別检查",
  "nav.crates": "模組架構",
  "nav.lang": "中文",
  "bench.title": "效能基準測試",
  "bench.zone_jit": "即時編譯",
  "bench.zone_interp": "解释執行",
  "bench.desc":
    "ulua 在解释執行与即時編譯模式下，与官方 <code>mlua (C++ Luau)</code>、LuaJIT 及 Lua 5.4 的效能对比。",
  "bench.env_label": "測試環境",
  "bench.tab_overview": "几何平均耗时",
  "bench.tab_single": "單項基準明細",
  "bench.col_bench": "基準項",
  "bench.toggle_table_show": "展开資料表格 ▼",
  "bench.toggle_table_hide": "收起資料表格 ▲",
  "bench.tip_faster": "越快越好",
  "bench.current": "本專案",
  "bench.lang.ulua": "Luau (纯 Rust)",
  "bench.lang.mlua_luau": "Luau (C++)",
  "bench.lang.mlua_luajit": "LuaJIT 2.1",
  "bench.lang.mlua_lua54": "Lua 5.4",
  "bench.item.fib": "斐波那契遞迴",
  "bench.item.fib_desc": "深度函式遞迴与调用栈开销",
  "bench.param.fib": "计算規模: 遞迴深度 N = 33",
  "bench.item.nbody": "天體引力模拟",
  "bench.item.nbody_desc": "重度浮點运算与局部寄存器调度",
  "bench.param.nbody": "计算規模: 450,000 步多體轨道积分模拟",
  "bench.item.mandel": "曼德博分形集合",
  "bench.item.mandel_desc": "密集迴圈分支与复平面浮點疊代",
  "bench.param.mandel": "计算規模: 400 × 400 复平面网格，256 次疊代",
  "bench.item.matmul": "稠密矩阵乘法",
  "bench.item.matmul_desc": "二维嵌套表读寫与陣列遍历吞吐",
  "bench.param.matmul": "计算規模: 320 × 320 矩阵相乘，O(N³) 次运算",
  "bench.item.tablesort": "陣列快速排序",
  "bench.item.tablesort_desc": "表元素快速排序与比较閉包调用",
  "bench.param.tablesort": "计算規模: 140,000 个伪随机數原地排序",
  "bench.item.strings": "字串格式化与拼接",
  "bench.item.strings_desc": "字串驻留、短串分配与垃圾回收吞吐",
  "bench.param.strings": "计算規模: 320,000 次字串格式化与拼接",
  "bench.item.binarytrees": "二叉树 GC 回收",
  "bench.item.binarytrees_desc": "高频树节點动態分配与 GC 回收吞吐",
  "bench.param.binarytrees": "计算規模: 最大深度 13 层树結構分配与 GC 回收",
  "bench.item.spectralnorm": "特徵值谱范數",
  "bench.item.spectralnorm_desc": "密集數学疊代与陣列内聯计算",
  "bench.param.spectralnorm": "计算規模: 1000 階矩阵近似特徵值疊代",
  "hero.title_pre": "用 Rust，",
  "hero.title_grad": "重寫 Luau",
  "pg.example": "示例",
  "pg.run": "執行",
  "pg.check": "型別检查",
  "pg.clear": "清除",
  "pg.editor_tab": "source.luau",
  "pg.diag_output": "診斷与輸出",
  "pg.loading_engine": "正在載入 Luau WebAssembly 引擎…",
  "pg.ready_prompt":
    "引擎已就緒。點擊「執行」(Ctrl+Enter) 執行脚本，或编辑程式碼體驗即時型別检查。",
  "pg.note":
    '停止输入后自动触发<strong>型別检查</strong>（點擊 <span class="kbd-inline">Lnn</span> 跳轉对应行）；按 <kbd>Ctrl+Enter</kbd> 快捷<strong>執行</strong>。',
  "about.title": "什么是 Luau？為何用 Rust 重寫",
  "about.lead":
    '<a href="https://luau.org" target="_blank" rel="noopener" class="luau-link">Luau ↗</a> 是 Roblox 开发的渐進型別嵌入式脚本語言，衍生自 Lua 5.1。在保留 Lua 轻量特性的同时，增加了靜態型別推導、語法擴充与效能最佳化。',
  "about.text":
    '官方 Luau 采用 C++ 實现。<a href="https://crates.io/crates/ulua" target="_blank" rel="noopener" class="crate-link"><strong>ulua</strong> ↗</a> 遵循 Luau 規範，使用 Rust 實现了解析器、編譯器、虛擬機与型別检查器，具备記憶體安全保障、mlua 风格 API 以及 WebAssembly 支援。',
  "about.p1_title": "記憶體安全",
  "about.p1_desc": "依靠 Rust 所有权模型，在編譯期消除悬垂指標与記憶體越界问題。",
  "about.p2_title": "開箱即用",
  "about.p2_desc":
    '通过 <a href="https://crates.io/crates/ulua" target="_blank" rel="noopener" class="crate-code-link"><code>cargo add ulua</code> ↗</a> 直接引入，無需 C/C++ 編譯工具鏈，提供对標 mlua 的绑定介面。',
  "about.p3_title": "支援原生与 WASM",
  "about.p3_desc":
    "既可作为脚本引擎嵌入 Rust 伺服器端或桌面应用，也可編譯为 WebAssembly 在瀏覽器中執行。",
  "features.title": "支援編譯为 WASM",
  "features.desc":
    "編譯器、虛擬機与型別推導引擎均支援編譯为 WebAssembly，在瀏覽器端直接執行与校验。",
  "features.f1_title": "終端側型別診斷",
  "features.f1_desc": "在瀏覽器中直接完成型別推導与行級診斷，無需请求遠端服务。",
  "features.f2_title": "纯 Rust 實现",
  "features.f2_desc": "无外部 C/C++ 依赖，開箱即用，易于与 Rust 專案无缝整合。",
  "features.f3_title": "沙盒隔離与離線執行",
  "features.f3_desc": "程式碼在瀏覽器本地沙盒中執行，無需網路連接，保障資料隱私。",
  "syntax.expand_all": "展開全部",
  "syntax.collapse_all": "折疊全部",
  "syntax.lua_title": "Lua 5.1 語法",
  "syntax.luau_title": "Luau 擴充",
  "syntax.lua_intro":
    "Lua 5.1 以轻量与簡潔著称，具备表結構、一等函式、词法閉包与協程等核心特性。ulua 完整兼容 Lua 5.1 語法規範。",
  "syntax.luau_intro":
    "Luau 在兼容 Lua 5.1 的基础上增加了渐進靜態型別系統、向量与緩衝區型別、現代控制流程語法及原生編譯最佳化。",
  "syntax.lua_basics": "變數、作用域与標量",
  "syntax.lua_variables_title": "局部變數与詞法作用域",
  "syntax.lua_variables_desc":
    "變數預設具有詞法作用域，推荐顯式宣告 local；支援 do...end 块級作用域遮蔽与全局環境 _G 交互：",
  "syntax.lua_types_title": "基础標量型別与執行期检測",
  "syntax.lua_types_desc":
    "Lua 5.1 具备动態弱型別系統，標量包括 nil, boolean, number, string；使用内置函式 type(v) 進行动態型別判別：",
  "syntax.lua_strings_title": "字串字面量与长括号語法",
  "syntax.lua_strings_desc":
    "支援單双引号常規字串与 [[...]] 多行原始字串；使用 .. 運算子進行字串拼接，# 運算子获取字节长度：",
  "syntax.lua_control": "控制流程与迴圈疊代",
  "syntax.lua_conditionals_title": "條件分支与真假值規則",
  "syntax.lua_conditionals_desc":
    '標準 if-then-elseif-else 結構；Lua 中仅 false 与 nil 视为假，數值 0、空字串 "" 与空表 {} 均为真值：',
  "syntax.lua_loops_title": "條件迴圈与數值型 for 迴圈",
  "syntax.lua_loops_desc":
    "包含 while 迴圈、后置條件的 repeat...until 迴圈，以及指定起始、终止和步长的數值型 for 迴圈：",
  "syntax.lua_pairs_title": "经典泛型疊代器 pairs 与 ipairs",
  "syntax.lua_pairs_desc":
    "pairs 用于遍历哈希表的全部键值对；ipairs 从索引 1 开始顺序遍历連續正整數序列，遇到第一个 nil 终止：",
  "syntax.lua_break_title": "break 跳出与末尾語句限制",
  "syntax.lua_break_desc":
    "break 用于跳出最内层迴圈；在標準語法中 break 必须位于程式碼块末尾，中间強行跳出需包裹 do break end：",
  "syntax.lua_functions": "函式机制与词法閉包",
  "syntax.lua_functions_title": "一級函式与多回傳值解構",
  "syntax.lua_functions_desc":
    "函式是一等公民，支援多回傳值展开与截斷；特別註意用括号 (fn()) 可无條件強制單值截斷：",
  "syntax.lua_varargs_title": "可變长參數机制 ...",
  "syntax.lua_varargs_desc":
    "通过 ... 接收任意數量的實參；使用 select('#', ...) 获取參數數量，或用 { ... } 将其打包为表：",
  "syntax.lua_closures_title": "词法閉包与環境状態封裝",
  "syntax.lua_closures_desc": "内部函式可引用外层局部變數（Upvalue），在閉包间共享状態：",
  "syntax.lua_errors_title": "保護模式与调用栈捕获",
  "syntax.lua_errors_desc":
    "Lua 无 try-catch 結構；通过 pcall 執行安全受控调用，或用 xpcall 挂接错误处理器在栈展开前捕获 debug.traceback：",
  "syntax.lua_tables": "万能表与元表系統",
  "syntax.lua_tables_title": "萬物皆表：陣列与哈希字典",
  "syntax.lua_tables_desc":
    "表是 Lua 唯一的复合資料結構，同时承担陣列、字典与对象；支援 # 长度運算子与 table.insert/remove/concat：",
  "syntax.lua_metatables_title": "元表系統与事件分发",
  "syntax.lua_metatables_desc":
    "利用 setmetatable 关聯元表，通过 __index 回退查询、__newindex 寫入拦截与 __call 仿函式定制行为：",
  "syntax.lua_weak_tables_title": "弱引用表与缓存机制",
  "syntax.lua_weak_tables_desc":
    "元表 __mode 屬性支援 'k'（弱键）、'v'（弱值）或 'kv'，无強引用时垃圾回收器自动回收键值对：",
  "syntax.lua_op_overload_title": "算术与关系運算子重载",
  "syntax.lua_op_overload_desc":
    "在元表中定義 __add, __sub, __mul, __eq, __tostring 等元方法，让自定義資料結構支援原生運算子：",
  "syntax.lua_oop_title": "原型物件導向与冒号語法糖",
  "syntax.lua_oop_desc":
    "利用 __index 查找回退机制建置原型继承鏈；使用 object:method() 語法糖自动隐式传递 self 接收者：",
  "syntax.lua_coroutines": "协作式多任务与協程",
  "syntax.lua_coroutines_title": "协作式協程生命周期",
  "syntax.lua_coroutines_desc":
    "包含 suspended、running、normal、dead 四种状態，通过 yield 与 resume 進行协作式任务切换与雙向資料传递：",
  "syntax.lua_generator_title": "協程状態生成器与自定義疊代器",
  "syntax.lua_generator_desc":
    "使用 coroutine.wrap 将生成器協程直接包裝为閉包疊代器，以同步书寫方式實现流式資料产出：",
  "syntax.luau_syntax": "現代語言語法糖",
  "syntax.compound_title": "复合赋值与整除运算",
  "syntax.compound_desc":
    "支援 +=, -=, *=, /=, //=, %=, ^=, ..= 語句；複雜左侧運算式嚴格仅求值一次；原生 // 向下整除：",
  "syntax.number_lit_title": "現代數值字面量与數字分隔符",
  "syntax.number_lit_desc":
    "統一 64 位雙精度浮點數，支援十六進位 0x、二進位 0b、科学计數法与下划线 _ 數字分隔符：",
  "syntax.string_escape_title": "擴充轉義字符与空白消除",
  "syntax.string_escape_desc":
    "支援 \\xHH 十六進位字符、\\u{...} 變长 Unicode 码點编码，以及行尾折行 \\z 吞噬跨行空白与缩進：",
  "syntax.interp_title": "模板字串插值与語法約束",
  "syntax.interp_desc": "反引号模板字串内嵌 {expr} 運算式求值，支援多行文本与轉義：",
  "syntax.const_binding_title": "常量绑定与不可變性語義",
  "syntax.const_binding_desc":
    "const 宣告不可重新赋值的變數绑定，配合 table.freeze 可實现浅层与深层只读：",
  "syntax.control_flow_title": "流程控制与 continue 跳轉",
  "syntax.control_flow_desc":
    "標準 if-then-elseif-else、while、repeat..until 与 for 迴圈，支援 continue 語句進入下一轮疊代：",
  "syntax.ifexpr_title": "條件分支運算式（三元替代）",
  "syntax.ifexpr_desc": "運算式形式的條件分支，必须包含 else 子句：",
  "syntax.iter_title": "通用迴圈疊代与 __iter 元方法",
  "syntax.iter_desc":
    "無需 pairs/ipairs 即可直接 for in 遍历表，保证連續陣列追加顺序，支援自定義 __iter 元方法：",
  "syntax.luau_types": "渐進靜態型別系統",
  "syntax.type_modes_title": "型別推斷模式与脚本編譯指令",
  "syntax.type_modes_desc":
    "首行註释指令，包括 --!strict 嚴格模式、--!nonstrict 寬容模式、--!nocheck 禁用模式与 --!native 原生机器码：",
  "syntax.type_annot_title": "基础型別註解与頂底型別",
  "syntax.type_annot_desc":
    "顯式標註 number, string, boolean, nil, thread, buffer, vector 以及 any, unknown, never：",
  "syntax.func_types_title": "函式型別籤名与形參提示",
  "syntax.func_types_desc":
    "以 (A, B) -> (R1, R2) 定義籤名，支援多回傳值、() 无回傳值以及用于文件提示的命名形參：",
  "syntax.union_inter_title": "聯合型別、可選型別与交叉型別",
  "syntax.union_inter_desc":
    "使用 | 表达多型別聯合，使用 ? 宣告可空型別；使用 & 组合多个表介面或定義重载函式籤名：",
  "syntax.casts_title": "強制型別斷言与推斷反射",
  "syntax.casts_desc":
    "通过 :: 運算子安全斷言轉换型別，使用 typeof() 在編譯期提取複雜運算式的推導結構：",
  "syntax.luau_tables": "結構化表与型別收窄",
  "syntax.tables_types_title": "結構化表型別与读寫控制",
  "syntax.tables_types_desc":
    "支援屬性定義、{T} 陣列簡寫、字典索引籤名以及 read / write 細粒度屬性读寫访问控制：",
  "syntax.table_states_title": "表状態演進与寬度子型別",
  "syntax.table_states_desc":
    "字面量未密封表可动態添加屬性，离开作用域或顯式標註后自动密封，密封表支援寬度子型別（超集兼容）：",
  "syntax.refinements_title": "標籤聯合与型別流細化",
  "syntax.refinements_desc":
    "通过字面量標籤区分資料分支，利用 type()、assert() 或條件分支在控制流程中自动收窄型別：",
  "syntax.tables_freeze_title": "表只读凍結与防護",
  "syntax.tables_freeze_desc": "使用 table.freeze 凍結表，禁止修改或新增欄位：",
  "syntax.luau_generics": "泛型与型別包",
  "syntax.generics_title": "泛型型別別名与形參預設值",
  "syntax.generics_desc":
    "通过 type 与 export type 宣告型別別名，支援函式泛型形參以及型別形參預設值：",
  "syntax.instantiate_title": "泛型顯式實例化 <<T>>",
  "syntax.instantiate_desc":
    "在调用处使用 <<Type>> 顯式传递型別實參，解决空容器工厂推斷歧義与字面量單例收窄，編譯期直接擦除：",
  "syntax.type_packs_title": "泛型型別包与同構變參",
  "syntax.type_packs_desc":
    "使用 T... 語法定義變长型別列表以實现參數完美轉发，与同構定型別可變參數 ...T 形成明确互补：",
  "syntax.luau_host": "物件導向与工程模組",
  "syntax.oop_title": "強型別物件導向元表模式",
  "syntax.oop_desc":
    "結合 setmetatable 与 typeof 實现強型別类定義，精準約束方法中 self 接收者的型別：",
  "syntax.modules_export_title": "模組隔離与導出語法",
  "syntax.modules_export_desc":
    "型別別名預設文件私有，export type 供外部 require 引用；新特性 export local/function 導出值：",
  "syntax.declare_title": "外部宣告語法与宿主契約",
  "syntax.declare_desc":
    "declare global、declare function 与 declare extern type，用于描述外部宿主環境註入的全局符号与类定義：",
  "syntax.luau_native": "底层原生与效能加速",
  "syntax.vector_title": "原生向量与硬體加速",
  "syntax.vector_desc": "内置 3D/4D 向量型別与 SIMD 加速计算，支援分量算术运算与整除：",
  "syntax.buffer_title": "原生連續記憶體緩衝區",
  "syntax.buffer_desc": "提供連續記憶體块读寫，支援各类數值型別读寫、字串轉换与記憶體拷贝：",
  "syntax.attr_title": "函式屬性註解",
  "syntax.attr_desc":
    "内置編譯器指令：@native 本地机器码編譯（非遞迴）、@checked 執行期參數斷言、@[deprecated {...}] 參數化警告：",
  "syntax.type_func_title": "編譯期型別元函式",
  "syntax.type_func_desc":
    "在靜態分析階段執行的元编程型別函式，调用内置 types 库动態检查、變换与生成新型別：",
  "embed.title": "在 Rust 中使用",
  "embed.desc":
    '<code>ulua-rt</code> 提供对標 <a href="https://github.com/mlua-rs/mlua" target="_blank" rel="noopener">mlua</a> 的安全 API，支援向 Luau 暴露 Rust 函式与自定義型別，异常会自动轉换为可捕获的 Lua 错误。纯 Rust 實现，无 C/FFI 依赖，支援原生与 WebAssembly。',
  "checker.title": "原生搭载靜態型別检查器",
  "checker.desc": "提供靜態型別检查器，支援在執行前對照宿主宣告校验程式碼：",
  "crates.title": "模組架構",
  "crates.desc": "ulua 采用模組化设计发布为独立的 Crate，可按需引入对应层級。",
  "crates.ulua":
    "入口 Crate，提供 mlua 风格 API 与 <code>compile</code>/<code>eval</code>/<code>check</code> 頂层函式，重導出各子模組",
  "crates.ulua-rt":
    "<strong>mlua 风格執行期绑定</strong>，包含 <code>Lua</code>、<code>create_function</code>、<code>UserData</code>、<code>FromLua</code>/<code>IntoLua</code>",
  "crates.ulua-common":
    "底层基建：<code>SmallVector</code>、<code>DenseHashMap</code>、<code>Variant</code>、FastFlags 开关",
  "crates.ulua-ast": "词法分析、語法解析器与抽象語法树（AST）",
  "crates.ulua-bytecode": "位元組碼指令規範与建置器",
  "crates.ulua-compiler": "Luau 源码至位元組碼編譯器",
  "crates.ulua-codegen": "原生机器码生成后端（A64 / X64）",
  "crates.ulua-vm": "寄存器虛擬機与完整標準库",
  "crates.ulua-analysis": "雙向靜態型別检查器与型別推斷引擎",
  "crates.ulua-config": "<code>.uluac</code> 配置文件解析",
  "crates.ulua-require": "基于字串模組路径的 require 依赖解析",
  "crates.ulua-web": "<code>wasm32</code> 绑定 —— 在瀏覽器沙盒中執行与型別检查 Luau",
  "crates.foot":
    "提供命令行工具：<code>ulua-repl-cli</code>（交互式 REPL）、<code>ulua-analyze-cli</code>（型別检查器）及編譯工具集。",
  "foot.license":
    "MIT 协议。ulua 是 Luau（© Roblox Corporation）的重寫專案，Luau 衍生自 Lua（© Lua.org, PUC-Rio）；完整保留上游版权資訊。",
  "foot.commit": "页面演示与編譯器均在瀏覽器 WebAssembly 環境中執行。",
  "foot.source": "源码与问題反馈：",
  "example.hello": "Hello, world（基础語法）",
  "example.fibonacci": "斐波那契數列（遞迴与疊代）",
  "example.tables": "表結構（陣列与字典）",
  "example.metatables": "元表（物件導向与運算子）",
  "example.strings": "字串（標準字串库）",
  "example.coroutines": "協程（协同生成器）",
  "example.typed": "渐進型別（靜態型別標註）",
  "example.globals": "遍历 _G（全局環境表）",
  "example.type_error": "型別错误（體驗型別检查器！）",
  "status.ready": "就緒",
  "status.running": "正在執行…",
  "status.checking": "正在检查…",
  "status.loading_wasm": "正在載入 WebAssembly…",
  "status.wasm_failed": "WebAssembly 引擎載入失败",
  "status.runtime_error": "執行期错误",
  "status.error": "错误",
  "status.no_errors": "无错误",
  "status.ran_ok": "執行成功",
  "status.errors": "个型別错误",
  "out.no_errors": "型別检查通过，未发现任何型別错误。",
  "out.no_output": "(无輸出 — 脚本未产生任何 print 内容)",
  "out.wasm_failed": "WebAssembly 引擎載入失败。\n",
};
