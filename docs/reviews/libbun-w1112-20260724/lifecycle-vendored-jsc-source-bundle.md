# Vendored JSC lifecycle source bundle

Exact product SHA: 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb

Exact product tree: cb964de8ab8162449fbe95959bf34d231570aa5c

This bundle binds VM construction, global-object access, event-loop drain, termination request/reset, worker teardown, and C++ VM calls. The C++ excerpt proves JSC__VM__deinit has an empty body; it cannot prove process death, containment drain, output joins, or retirement. Cooperative termination reset is therefore reusable only after the owner proves complete invocation and output drain independently.

## Full-file identities

| Path | Git blob | SHA-256 | Bytes |
| --- | --- | --- | ---: |
| vendor/bun/src/jsc/VirtualMachine.rs | 0ee5e0b90a15caa28dd64a0b4e6d4de998fd91dc | aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a | 296637 |
| vendor/bun/src/jsc/VM.rs | 47398ab74d014061a65b17fa7201d85ff4489778 | 47aa4f3b8642dfefe21dd56c2fa446889cd701cc3e9fdbf7d962bf4f6aee7119 | 8822 |
| vendor/bun/src/jsc/JSGlobalObject.rs | bc6ab6214c903a2467e2f60bdf9ddea2b7f35fcf | d6dc512fb3021f0c57bee74fe66230eaebf5f7b94f3bf75602af0ea3a045786e | 69350 |
| vendor/bun/src/jsc/virtual_machine_exports.rs | 658114e48f264091ca8a2d2e4d5d13dfca86b010 | 95946faae0cd89ac0ccf1d037312e307a88742beed7e5ac2770f91910582b23b | 13386 |
| vendor/bun/src/jsc/bindings/bindings.cpp | b08737cc9ba97c08fccd8d291f73cc03275031d0 | e32cd326cc1592bed6f70bff8eba95bfd855a17a6998567976b352e9478d5bff | 255398 |
| vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp | a7969dc80971bbd085baaa8178a9e0a1a765cfa1 | 72f4a089c9e75ad919be6ebe8d47e846db74b1872aecdad7f9b217e3c696cf9d | 185486 |
| vendor/bun/src/jsc/bindings/ZigGlobalObject.h | ec74c45e6c49b4ae8a204387c3bfc2bb285338ea | db333158e15da0eb23da28fca12255d3a6850e902028ce7fba486bd623dbe282 | 55856 |
| vendor/bun/src/jsc/VirtualMachine.zig | 461906c6b3aaafae04772b69827f4beaca50b46e | 8ce0ae2b45cec903d949ef52d0dcdeb2abb3b9bbe910aa644f99420cf8e9d667 | 162376 |

## vendor/bun/src/jsc/VirtualMachine.rs:605-810

- Full-file Git blob: 0ee5e0b90a15caa28dd64a0b4e6d4de998fd91dc
- Full-file SHA-256: aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a
- Full-file bytes: 296637
- Excerpt line span: 605-810
- Excerpt SHA-256: b753eecd1876ecc137393241ae98ba015c537081ce2f91e9037e77dd1eff416f

   605  impl VirtualMachine {
   606      /// Safe `&'static` accessor for the current thread's VM. The VM is a
   607      /// per-thread singleton allocated once in [`init`] and never freed until
   608      /// thread teardown, so the `'static` lifetime is sound. Mutation goes
   609      /// through [`JsCell`]-wrapped fields (`vm.field.with_mut(|x| ...)`);
   610      /// legacy code that still needs `&mut VirtualMachine` whole-struct uses
   611      /// [`Self::get_mut_ptr`] + an explicit `unsafe` deref.
   612      #[inline(always)]
   613      pub fn get() -> &'static VirtualMachine {
   614          // SAFETY: `get_or_null()` returns the thread-local pointer set by
   615          // `init()`; non-null while a VM is installed; the allocation outlives
   616          // the thread.
   617          unsafe { &*Self::get_mut_ptr() }
   618      }
   619
   620      /// Raw `*mut` accessor for the current thread's VM. Prefer [`Self::get`]
   621      /// for read access and `JsCell` field projection for mutation; this exists
   622      /// for the (shrinking) set of call sites that still take
   623      /// `*mut VirtualMachine` or need a whole-struct `&mut`.
   624      ///
   625      /// Per-request hot path: `vm_get`/`as_mut`/`NewServer::vm_mut` all funnel
   626      /// through here, so it is reached several times per `run_callback` (Zig
   627      /// just reads the bare `threadlocal var vm`). The previous `.expect()`
   628      /// emitted a check + cold-path panic-format branch on every call;
   629      /// `unwrap_unchecked` collapses to one TLS load. The "no VM on this
   630      /// thread" case is a programmer error (host_fn reached before `init()`),
   631      /// not a recoverable condition — keep the diagnostic in debug builds only.
   632      #[inline(always)]
   633      pub fn get_mut_ptr() -> *mut VirtualMachine {
   634          debug_assert!(
   635              Self::get_or_null().is_some(),
   636              "VirtualMachine.get() called with no VM on this thread",
   637          );
   638          // SAFETY: every caller is reached from a JS host_fn / event-loop tick,
   639          // which by construction runs after `init()` installed `VMHolder::VM`
   640          // for this thread.
   641          unsafe { Self::get_or_null().unwrap_unchecked() }
   642      }
   643
   644      /// `&mut self` from `&self` — the `JsCell` escape hatch applied to the
   645      /// whole VM. Exists so legacy `&mut VirtualMachine`-taking helpers can be
   646      /// called from a safe `&'static VirtualMachine` without an `unsafe` block
   647      /// at every call site. Same single-JS-thread soundness contract as
   648      /// [`JsCell::get_mut`]; keep the borrow short and do not hold across
   649      /// reentrant JS calls.
   650      /// Routes through [`Self::get_mut_ptr`] (the thread-local raw pointer)
   651      /// rather than casting `&self`, so provenance is the original `*mut`
   652      /// allocation — avoids the `invalid_reference_casting` UB lint.
   653      #[inline(always)]
   654      #[allow(clippy::mut_from_ref)]
   655      pub fn as_mut(&self) -> &mut VirtualMachine {
   656          debug_assert!(core::ptr::eq(self, Self::get_mut_ptr()));
   657          // SAFETY: single-JS-thread invariant — see `unsafe impl Sync` above.
   658          // Provenance comes from the thread-local `*mut` set in `init()`.
   659          unsafe { &mut *Self::get_mut_ptr() }
   660      }
   661
   662      /// `&'static mut` to this thread's VM singleton — the static-fn counterpart
   663      /// of [`Self::as_mut`]. Exists so per-type `fn vm_mut(&self)` shims (sql,
   664      /// bake, cron) collapse to one call instead of each open-coding
   665      /// `unsafe { &mut *self.vm.as_ptr() }` against a stored `BackRef`.
   666      ///
   667      /// Returns `'static` (not tied to any `&self`) so callers may pair the VM
   668      /// borrow with a disjoint `&mut self.field` in the same expression. Same
   669      /// single-JS-thread soundness contract as [`JsCell::get_mut`] and
   670      /// [`Self::as_mut`]: keep the borrow short and do not hold it across
   671      /// reentrant JS calls. Provenance is the thread-local `*mut` installed by
   672      /// `init()`, so this is sound regardless of how the caller's own
   673      /// `BackRef<VirtualMachine>` was constructed.
   674      #[inline(always)]
   675      pub fn get_mut() -> &'static mut VirtualMachine {
   676          // SAFETY: single-JS-thread invariant — see `unsafe impl Sync` above.
   677          // Provenance comes from the thread-local `*mut` set in `init()`.
   678          unsafe { &mut *Self::get_mut_ptr() }
   679      }
   680
   681      #[inline(always)]
   682      pub fn get_or_null() -> Option<*mut VirtualMachine> {
   683          // thread-local set by init() on this thread; one VM per thread
   684          VM.get()
   685      }
   686
   687      pub fn get_main_thread_vm() -> Option<*mut VirtualMachine> {
   688          let p = MAIN_THREAD_VM.load(core::sync::atomic::Ordering::Acquire);
   689          if p.is_null() { None } else { Some(p) }
   690      }
   691
   692      #[inline]
   693      pub fn is_loaded() -> bool {
   694          VM.get().is_some()
   695      }
   696
   697      /// Installs `vm` as the current thread's VM (Zig: `VMHolder.vm = vm`).
   698      pub fn set_current(vm: *mut VirtualMachine) {
   699          VM.set(Some(vm));
   700      }
   701
   702      /// Returns `&'static` so callers can hold the global across `&mut self`
   703      /// reborrows (`JSGlobalObject` is a separate JSC heap allocation, so no
   704      /// overlap with `VirtualMachine` storage). Same `'static`-on-the-JS-thread
   705      /// contract as [`JSGlobalObject::bun_vm`] — the global lives for the VM
   706      /// lifetime, and the VM is the per-thread singleton.
   707      #[inline(always)]
   708      pub fn global(&self) -> &'static JSGlobalObject {
   709          // `global` is set during init and live for the VM lifetime.
   710          // `JSGlobalObject` is an `opaque_ffi!` ZST handle; `opaque_ref` is the
   711          // centralised non-null-ZST deref proof.
   712          JSGlobalObject::opaque_ref(self.global)
   713      }
   714
   715      /// Spec VirtualMachine.zig: `pub fn eventLoop(this: *VirtualMachine) *EventLoop`
   716      /// — returns a raw `*EventLoop` (no aliasing guarantee). Returning `&mut`
   717      /// here would let two overlapping callers (e.g. a JS callback re-entering
   718      /// `vm.event_loop()` from inside `tick()`) mint aliased `&mut EventLoop` to
   719      /// the same allocation — UB per PORTING.md §Forbidden. Callers form a
   720      /// short-lived `&mut *p` at the use site instead, mirroring [`Self::get`].
   721      #[inline(always)]
   722      pub fn event_loop(&self) -> *mut EventLoop {
   723          // self-pointer to regular_event_loop or macro_event_loop
   724          self.event_loop
   725      }
   726
   727      /// Safe `&mut EventLoop` accessor — the [`JsCell`] escape hatch applied to
   728      /// the active event loop. `event_loop` is a self-pointer into either
   729      /// `regular_event_loop` or `macro_event_loop` (both owned by this VM), so it
   730      /// is live for the VM lifetime. Same single-JS-thread soundness contract as
   731      /// [`Self::as_mut`]; keep the borrow short and do not hold across reentrant
   732      /// JS calls. Prefer this over `unsafe { &mut *vm.event_loop() }` at call
   733      /// sites.
   734      #[inline(always)]
   735      #[allow(clippy::mut_from_ref)]
   736      pub fn event_loop_mut(&self) -> &mut EventLoop {
   737          // SAFETY: `event_loop` points at a sibling field of this VM; non-null
   738          // after `init()`; single-JS-thread invariant per `unsafe impl Sync`.
   739          unsafe { &mut *self.event_loop }
   740      }
   741
   742      /// Safe `&EventLoop` accessor — shared variant of [`Self::event_loop_mut`].
   743      /// Prefer when only reading event-loop fields (queue lengths, pending
   744      /// refs) to avoid minting an unnecessary `&mut`.
   745      #[inline(always)]
   746      pub fn event_loop_shared(&self) -> &EventLoop {
   747          // SAFETY: see `event_loop_mut`.
   748          unsafe { &*self.event_loop }
   749      }
   750
   751      /// Alias for [`Self::event_loop_mut`]. Kept for callers migrated on the
   752      /// `runtime-hostfn-safe` branch; both names funnel into the single audited
   753      /// `unsafe` deref above.
   754      #[inline(always)]
   755      #[allow(clippy::mut_from_ref)]
   756      pub fn event_loop_ref(&self) -> &mut EventLoop {
   757          self.event_loop_mut()
   758      }
   759
   760      /// Safe `&VM` accessor for the JSC VM owned by this Bun VM. Set once in
   761      /// `init()` and live for the VM lifetime.
   762      #[inline(always)]
   763      pub fn jsc_vm(&self) -> &VM {
   764          // `jsc_vm` set in `init()`, valid for VM lifetime. `VM` is an
   765          // `opaque_ffi!` ZST handle; `opaque_ref` is the centralised
   766          // non-null-ZST deref proof.
   767          VM::opaque_ref(self.jsc_vm)
   768      }
   769
   770      /// Safe `&mut VM` accessor for the JSC VM. Set once in `init()` and live
   771      /// for the VM lifetime; the JSC `VM` lives in a separate heap allocation
   772      /// so this never aliases another field of `self`.
   773      #[inline]
   774      pub fn jsc_vm_mut(&mut self) -> &mut VM {
   775          // `jsc_vm` set in `init()`, valid for VM lifetime. `VM` is an
   776          // `opaque_ffi!` ZST handle; `opaque_mut` is the centralised
   777          // non-null-ZST deref proof (zero-byte `&mut` cannot alias).
   778          VM::opaque_mut(self.jsc_vm)
   779      }
   780
   781      /// Raw accessor for the hot-reload import watcher. `bun_watcher` is the
   782      /// type-erased `*mut ImportWatcher` installed by
   783      /// [`crate::hot_reloader::HotReloaderCtx::install_bun_watcher`] (separate
   784      /// `Box` heap allocation), or null when hot reload is disabled.
   785      ///
   786      /// NOTE: unlike `event_loop_mut`, the pointee is **not** JS-thread-only —
   787      /// the inner `Box<Watcher>` is held as `&mut Watcher` for the lifetime of
   788      /// the spawned file-watcher thread (`Watcher::thread_main`), and
   789      /// `RuntimeTranspilerStore` reads it from transpiler workers. The Zig spec
   790      /// models this as an alias-allowed `*Watcher` with an internal mutex, so we
   791      /// return the raw pointer and leave the `unsafe` deref at the call site to
   792      /// keep the cross-thread hazard visible. Callers must scope any reborrow to
   793      /// a single mutex-guarded `Watcher` operation.
   794      #[inline]
   795      pub fn bun_watcher_ptr(&self) -> *mut crate::hot_reloader::ImportWatcher {
   796          self.bun_watcher as *mut crate::hot_reloader::ImportWatcher
   797      }
   798
   799      /// `event_loop().enter()` now, `.exit()` on drop. Safe wrapper over
   800      /// [`EventLoop::enter_scope`] for the common `vm.event_loop()` case.
   801      #[inline]
   802      pub fn enter_event_loop_scope(&self) -> crate::event_loop::EventLoopEnterGuard {
   803          // SAFETY: `self.event_loop` is the live VM-owned event-loop pointer and
   804          // remains valid for the VM (and thus the guard's) lifetime.
   805          unsafe { EventLoop::enter_scope(self.event_loop) }
   806      }
   807
   808      /// Safe shared-reference accessor for the process-lifetime dotenv loader
   809      /// (`vm.transpiler.env`). The loader is allocated once during VM init and
   810      /// never freed; callers previously open-coded `unsafe { &*vm.transpiler.env }`.

## vendor/bun/src/jsc/VirtualMachine.rs:1170-1230

- Full-file Git blob: 0ee5e0b90a15caa28dd64a0b4e6d4de998fd91dc
- Full-file SHA-256: aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a
- Full-file bytes: 296637
- Excerpt line span: 1170-1230
- Excerpt SHA-256: fa9b28e17e7084de5c190ffac19e3c67d201367c0107fb19e945d654db9da8b7

  1170      }
  1171
  1172      pub fn enqueue_task(&mut self, task: bun_event_loop::Task) {
  1173          // accessed here (no overlapping `&mut EventLoop`).
  1174          self.event_loop_mut().enqueue_task(task);
  1175      }
  1176
  1177      pub fn tick(&mut self) {
  1178          self.event_loop_mut().tick();
  1179      }
  1180
  1181      #[inline(always)]
  1182      pub fn drain_microtasks(&mut self) {
  1183          let _ = self.event_loop_mut().drain_microtasks();
  1184      }
  1185
  1186      pub fn assert_on_js_thread(&self) {
  1187          #[cfg(debug_assertions)]
  1188          {
  1189              assert!(
  1190                  std::thread::current().id() == self.debug_thread_id,
  1191                  "VirtualMachine accessed from wrong thread"
  1192              );
  1193          }
  1194      }
  1195
  1196      /// `runWithAPILock(comptime Context, ctx, comptime fn)` — acquires the JSC
  1197      /// API lock around `f(ctx)`. Rust collapses the comptime params into a closure.
  1198      ///
  1199      /// Spec VirtualMachine.zig:2629-2631: `this.global.vm().holdAPILock(ctx, callback)`.
  1200      /// Routes `f` through `JSC__VM__holdAPILock` via an `OpaqueWrap`-style C
  1201      /// trampoline so the JSC API lock is held for the full duration of `f()`.
  1202      pub fn run_with_api_lock<F, R>(&self, f: F) -> R
  1203      where
  1204          F: FnOnce() -> R,
  1205      {
  1206          use core::mem::{ManuallyDrop, MaybeUninit};
  1207
  1208          // PORT NOTE: Zig's `OpaqueWrap(Context, function)` synthesizes a
  1209          // `fn(*anyopaque) void` that casts back and calls the body. The Rust
  1210          // closure carries its own context, so the trampoline state is just
  1211          // `{ closure, out-slot }`. `ManuallyDrop` lets us move the `FnOnce`
  1212          // out by value inside the `extern "C"` body without `Option::take`.
  1213          struct Trampoline<F, R> {
  1214              f: ManuallyDrop<F>,
  1215              result: MaybeUninit<R>,
  1216          }
  1217
  1218          extern "C" fn call<F: FnOnce() -> R, R>(ctx: *mut c_void) {
  1219              // SAFETY: `ctx` is `&mut Trampoline<F, R>` on the caller's stack;
  1220              // `JSC__VM__holdAPILock` invokes us exactly once with that pointer.
  1221              let t = unsafe { bun_ptr::callback_ctx::<Trampoline<F, R>>(ctx) };
  1222              // SAFETY: single-shot — `f` is taken exactly once.
  1223              let f = unsafe { ManuallyDrop::take(&mut t.f) };
  1224              t.result.write(f());
  1225          }
  1226
  1227          let mut t = Trampoline::<F, R> {
  1228              f: ManuallyDrop::new(f),
  1229              result: MaybeUninit::uninit(),
  1230          };

## vendor/bun/src/jsc/VirtualMachine.rs:1360-1570

- Full-file Git blob: 0ee5e0b90a15caa28dd64a0b4e6d4de998fd91dc
- Full-file SHA-256: aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a
- Full-file bytes: 296637
- Excerpt line span: 1360-1570
- Excerpt SHA-256: 1742606cee5c8aa6e3c2793165b6b1c822386f3fe29f88604daf19bf1d222d73

  1360      /// is unset.
  1361      pub fn ensure_debugger(&mut self, block_until_connected: bool) -> Result<(), bun_core::Error> {
  1362          if let Some(hooks) = runtime_hooks() {
  1363              // SAFETY: hook contract — `self` is the live per-thread VM.
  1364              unsafe { (hooks.ensure_debugger)(self, block_until_connected) };
  1365          }
  1366          Ok(())
  1367      }
  1368
  1369      /// Whether this VM should be destroyed after it exits, even if it is the
  1370      /// main thread's VM. Worker VMs are always destroyed on exit, regardless
  1371      /// of this setting. Setting this to true may expose bugs that would
  1372      /// otherwise only occur using Workers.
  1373      pub fn should_destruct_main_thread_on_exit(&self) -> bool {
  1374          bun_core::env_var::feature_flag::BUN_DESTRUCT_VM_ON_EXIT::get().unwrap_or(false)
  1375      }
  1376
  1377      pub fn uncaught_exception(
  1378          &mut self,
  1379          global_object: &JSGlobalObject,
  1380          err: JSValue,
  1381          is_rejection: bool,
  1382      ) -> bool {
  1383          if self.is_shutting_down() {
  1384              return true;
  1385          }
  1386
  1387          if isBunTest.load(core::sync::atomic::Ordering::Relaxed) {
  1388              self.unhandled_error_counter += 1;
  1389              (self.on_unhandled_rejection)(self, global_object, err);
  1390              return true;
  1391          }
  1392
  1393          let hooks = runtime_hooks().expect("RuntimeHooks not installed");
  1394          if self.is_handling_uncaught_exception {
  1395              self.run_error_handler(err, None);
  1396              // SAFETY: `global_object` is the live VM global; `process_exit` is
  1397              // `bun_runtime::node::process::exit` (main-thread `noreturn`).
  1398              unsafe { (hooks.process_exit)(global_object.as_ptr(), 7) };
  1399              panic!("Uncaught exception while handling uncaught exception");
  1400          }
  1401          if self.exit_on_uncaught_exception {
  1402              self.run_error_handler(err, None);
  1403              // SAFETY: see above.
  1404              unsafe { (hooks.process_exit)(global_object.as_ptr(), 1) };
  1405              panic!("made it past process.exit()");
  1406          }
  1407          self.is_handling_uncaught_exception = true;
  1408          let handled = Bun__handleUncaughtException(
  1409              global_object,
  1410              err.to_error().unwrap_or(err),
  1411              if is_rejection { 1 } else { 0 },
  1412          ) > 0;
  1413          if !handled {
  1414              // TODO maybe we want a separate code path for uncaught exceptions
  1415              self.unhandled_error_counter += 1;
  1416              self.exit_handler.exit_code = 1;
  1417              (self.on_unhandled_rejection)(self, global_object, err);
  1418          }
  1419          // PORT NOTE: Zig `defer this.is_handling_uncaught_exception = false;`
  1420          // (VirtualMachine.zig:707) covers BOTH the FFI call and the
  1421          // `onUnhandledRejection` callback above. The flag must stay raised
  1422          // while that callback runs so a re-entrant `uncaught_exception` from
  1423          // a user handler trips the recursion guard and hard-exits with code 7
  1424          // instead of recursing. Neither the FFI call nor the fn-pointer
  1425          // callback unwind past this frame (re-entry hits `process_exit` →
  1426          // `panic!`, which never returns), so a linear reset here matches the
  1427          // Zig `defer` scope.
  1428          self.is_handling_uncaught_exception = false;
  1429          handled
  1430      }
  1431
  1432      pub fn hot_map(&mut self) -> Option<&mut crate::rare_data::HotMap> {
  1433          if self.hot_reload != HOT_RELOAD_HOT {
  1434              return None;
  1435          }
  1436          // TODO(b2-cycle): spec lazy-inits via `RareData::hotMap(allocator)`;
  1437          // that accessor is gated in `rare_data.rs::_accessor_body`. Until it
  1438          // un-gates, return whatever the field already holds (callers that need
  1439          // the lazy-init path are themselves gated on `bun_runtime`).
  1440          self.rare_data.as_deref_mut()?.hot_map.as_mut()
  1441      }
  1442
  1443      pub fn on_before_exit(&mut self) {
  1444          ExitHandler::dispatch_on_before_exit(self);
  1445          let mut dispatch = false;
  1446          loop {
  1447              while self.is_event_loop_alive() {
  1448                  self.tick();
  1449                  self.auto_tick_active();
  1450                  dispatch = true;
  1451              }
  1452
  1453              if dispatch {
  1454                  ExitHandler::dispatch_on_before_exit(self);
  1455                  dispatch = false;
  1456
  1457                  if self.is_event_loop_alive() {
  1458                      continue;
  1459                  }
  1460              }
  1461
  1462              break;
  1463          }
  1464      }
  1465
  1466      pub fn on_exit(&mut self) {
  1467          // Write CPU profile if profiling was enabled - do this FIRST before any
  1468          // shutdown begins. Grab the config and null it out to make this
  1469          // idempotent.
  1470          if let Some(config) = self.cpu_profiler_config.take() {
  1471              if let Err(e) =
  1472                  crate::bun_cpu_profiler::stop_and_write_profile(self.jsc_vm_mut(), &config)
  1473              {
  1474                  bun_core::Output::err(bun_core::Error::from(e), "Failed to write CPU profile", ());
  1475              }
  1476          }
  1477          // Write heap profile if profiling was enabled - do this after CPU
  1478          // profile but before shutdown.
  1479          if let Some(config) = self.heap_profiler_config.take() {
  1480              if let Err(e) =
  1481                  crate::bun_heap_profiler::generate_and_write_profile(self.jsc_vm_mut(), config)
  1482              {
  1483                  bun_core::Output::err(e, "Failed to write heap profile", ());
  1484              }
  1485          }
  1486
  1487          ExitHandler::dispatch_on_exit(self);
  1488          self.is_shutting_down = true;
  1489
  1490          // Make sure we run new cleanup hooks introduced by running cleanup
  1491          // hooks.
  1492          // PORT NOTE: each iteration re-fetches `rare_data` so the FFI hook
  1493          // bodies (which may re-enter `VirtualMachine` and push more hooks) do
  1494          // not run while a `&mut RareData` is live — the borrow ends after
  1495          // `mem::take` returns the owned `Vec`.
  1496          loop {
  1497              let hooks = match self.rare_data.as_deref_mut() {
  1498                  Some(rare) if !rare.cleanup_hooks.is_empty() => {
  1499                      core::mem::take(&mut rare.cleanup_hooks)
  1500                  }
  1501                  _ => break,
  1502              };
  1503              for hook in hooks {
  1504                  (hook.func)(hook.ctx);
  1505              }
  1506          }
  1507          // Zig `defer rare_data.cleanup_hooks.clearAndFree(...)` — `mem::take`
  1508          // above leaves an empty `Vec` (capacity already freed by drop).
  1509      }
  1510
  1511      pub fn global_exit(&mut self) -> ! {
  1512          debug_assert!(self.is_shutting_down());
  1513          // FIXME: we should be doing this, but we're not, but unfortunately
  1514          // doing it causes like 50+ tests to break
  1515          // self.event_loop().tick();
  1516
  1517          if self.should_destruct_main_thread_on_exit() {
  1518              if let Some(t) = self.event_loop_mut().forever_timer.take() {
  1519                  // SAFETY: `t` is the live usockets timer created in
  1520                  // `EventLoop::auto_tick`; `close::<true>()` (fallthrough)
  1521                  // frees it without re-entering the loop. Spec
  1522                  // VirtualMachine.zig:967 `t.deinit(true)`.
  1523                  unsafe { uws::Timer::close::<true>(t.as_ptr()) };
  1524              }
  1525              // Detached worker threads may still be in startVM()/spin() using
  1526              // the process-global resolver BSSMap singletons. transpiler.deinit()
  1527              // below frees those singletons, so request termination of every
  1528              // live worker and wait for each to reach shutdown() first.
  1529              if let Some(hooks) = runtime_hooks() {
  1530                  // Main-thread only; futex-waits on every registered worker
  1531                  // until each unparks at shutdown().
  1532                  (hooks.terminate_all_workers_and_wait)(10_000);
  1533              }
  1534
  1535              // Embedded per-VM socket groups must drain while JSC is still
  1536              // alive (closeAll() fires on_close → JS). After JSC teardown,
  1537              // RareData's Drop only deinit()s the groups (asserts empty).
  1538              if self.rare_data.is_some() {
  1539                  // PORT NOTE: reshaped for borrowck — `close_all_socket_groups`
  1540                  // walks the loop's group list via `vm.uws_loop()` and never
  1541                  // touches `vm.rare_data`, so the disjoint reborrow is sound.
  1542                  // SAFETY: `self` is the live per-thread VM; the shared borrow
  1543                  // only reads `event_loop_handle` (no overlap with `rare_data`).
  1544                  let vm_ref = unsafe { &*core::ptr::from_ref(self) };
  1545                  self.rare_data
  1546                      .as_deref_mut()
  1547                      .unwrap()
  1548                      .close_all_socket_groups(vm_ref);
  1549              }
  1550
  1551              Zig__GlobalObject__destructOnExit(self.global());
  1552
  1553              // lastChanceToFinalize() above runs Listener/Server finalize →
  1554              // their own embedded group.closeAll() → sockets land in
  1555              // loop.closed_head. Drain again now or LSAN reports every accepted
  1556              // socket that was still open at process.exit().
  1557              // SAFETY: `uws::Loop::get()` returns the process-global usockets
  1558              // loop, which is live for the process lifetime.
  1559              unsafe { (*uws::Loop::get()).drain_closed_sockets() };
  1560
  1561              // TODO(port): `self.transpiler.deinit()` — `Transpiler<'_>` has no
  1562              // `deinit()` yet (resolver BSSMap teardown not ported).
  1563              self.gc_controller.deinit();
  1564              self.destroy();
  1565          }
  1566          bun_core::Global::exit(u32::from(self.exit_handler.exit_code))
  1567      }
  1568  }
  1569
  1570  extern crate alloc;

## vendor/bun/src/jsc/VirtualMachine.rs:1941-2180

- Full-file Git blob: 0ee5e0b90a15caa28dd64a0b4e6d4de998fd91dc
- Full-file SHA-256: aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a
- Full-file bytes: 296637
- Excerpt line span: 1941-2180
- Excerpt SHA-256: 7b80eb810244a201286ef06d6f0658dc26498f79d7049cedadc5f25aa54cf2e5

  1941  impl VirtualMachine {
  1942      /// `VirtualMachine.init(opts)` — allocate + wire the per-thread VM.
  1943      ///
  1944      /// PORT NOTE: every step that names a `bun_runtime` / `bun_webcore` type
  1945      /// (`Timer.All.init`, `Body.Value.HiveAllocator`, `configureDebugger`,
  1946      /// `Config.configureTransformOptionsForBunVM`, `ParentDeathWatchdog`) is
  1947      /// dispatched through `RuntimeHooks::init_runtime_state` so `bun_jsc` does
  1948      /// not name those types directly. The hook receives the boxed VM after the
  1949      /// JSC-tier fields are populated and finishes the rest.
  1950      pub fn init(mut opts: InitOptions) -> Result<*mut VirtualMachine, bun_core::Error> {
  1951          jsc::mark_binding();
  1952
  1953          // Spec VirtualMachine.zig:1234 — `opts.log orelse allocator.create(Log)`.
  1954          let log: *mut bun_ast::Log = match opts.log {
  1955              Some(l) => l.as_ptr(),
  1956              None => bun_core::heap::into_raw(Box::new(bun_ast::Log::default())),
  1957          };
  1958
  1959          // SAFETY: VM is large + self-referential; allocate zeroed and fill in
  1960          // place (mirrors Zig's `allocator.create` + struct-init). The
  1961          // allocation lives for the thread lifetime (never freed on the main
  1962          // thread; worker `destroy()` frees it explicitly).
  1963          //
  1964          // PORT NOTE (validity): the zeroed bytes are NOT a valid
  1965          // `VirtualMachine` — `origin_timer: Instant`, `on_unhandled_rejection:
  1966          // fn(...)`, (debug) `debug_thread_id: ThreadId`, every `Vec`/`Box`/
  1967          // `HashMap`/`ArrayHashMap` field (NonNull dangling-when-empty), `URL`
  1968          // (`&[u8]` references), and `Option<bool>` (bool-niche → zero = Some)
  1969          // have no all-zero repr. We therefore never materialize
  1970          // `&mut VirtualMachine` until all such fields have been `ptr::write`n
  1971          // via `addr_of_mut!`; remaining fields are zero-valid
  1972          // (integers/raw-ptr/atomic-mutex/`Option<NonNull>`/`Option<Box>`) so
  1973          // the zero-fill stands in for the Zig struct-init defaults.
  1974          let layout = core::alloc::Layout::new::<VirtualMachine>();
  1975          // SAFETY: `layout` is non-zero-sized; `alloc_zeroed` returns either a
  1976          // valid aligned ptr or null (handled by `handle_alloc_error`).
  1977          let vm: *mut VirtualMachine = unsafe {
  1978              let p = alloc::alloc::alloc_zeroed(layout);
  1979              if p.is_null() {
  1980                  alloc::alloc::handle_alloc_error(layout);
  1981              }
  1982              p.cast()
  1983          };
  1984          VM.set(Some(vm));
  1985          if opts.is_main_thread {
  1986              MAIN_THREAD_VM.store(vm, core::sync::atomic::Ordering::Release);
  1987          }
  1988
  1989          // ConsoleObject is self-referential (buffers + adapters) — allocate
  1990          // stable storage and init in place. Spec VirtualMachine.zig:1238-1239:
  1991          // `console.init(Output.rawErrorWriter(), Output.rawWriter())` must
  1992          // happen BEFORE the pointer is stored/passed; the previous port left
  1993          // it as raw `MaybeUninit` (UB on first C++ read).
  1994          let mut console_box: Box<core::mem::MaybeUninit<crate::console_object::ConsoleObject>> =
  1995              Box::new(core::mem::MaybeUninit::uninit());
  1996          crate::console_object::ConsoleObject::init_in_place(
  1997              &mut console_box,
  1998              bun_core::Output::raw_error_writer(),
  1999              bun_core::Output::raw_writer(),
  2000          );
  2001          let console =
  2002              bun_core::heap::into_raw(console_box).cast::<crate::console_object::ConsoleObject>();
  2003
  2004          let context_id = opts
  2005              .context_id
  2006              .unwrap_or(if opts.is_main_thread { 1 } else { i32::MAX });
  2007
  2008          // SAFETY: `vm` is a fresh unique zeroed allocation on this thread. All
  2009          // writes go through `addr_of_mut!` so no `&mut VirtualMachine` is
  2010          // formed while non-zero-valid fields are still zero. Every target is
  2011          // either zero-valid (no Drop on the overwritten bytes) or written via
  2012          // `ptr::write` (no Drop of the uninit bytes).
  2013          unsafe {
  2014              use core::ptr::addr_of_mut;
  2015              addr_of_mut!((*vm).global).write(core::ptr::null_mut());
  2016              addr_of_mut!((*vm).console).write(console);
  2017              // `log` is a fresh leaked Box; outlives the VM.
  2018              addr_of_mut!((*vm).log).write(NonNull::new(log));
  2019              addr_of_mut!((*vm).main).write(bun_ptr::RawSlice::EMPTY);
  2020              addr_of_mut!((*vm).main_hash).write(0);
  2021              addr_of_mut!((*vm).main_resolved_path).write(bun_core::String::empty());
  2022              addr_of_mut!((*vm).hide_bun_stackframes).write(true);
  2023              addr_of_mut!((*vm).is_main_thread).write(opts.is_main_thread);
  2024              // Spec VirtualMachine.zig:154 — `= std.math.maxInt(u32)`. Left at the
  2025              // zeroed default this aliases `hot_reload_counter`'s initial 0, so a
  2026              // watcher event that races the very first entry-point load makes
  2027              // `reload()` think the rejection was already reported and proceed
  2028              // (replacing `pending_internal_promise`) instead of deferring,
  2029              // dropping the error on the floor.
  2030              addr_of_mut!((*vm).pending_internal_promise_reported_at).write(u32::MAX);
  2031              addr_of_mut!((*vm).on_unhandled_rejection)
  2032                  .write(VirtualMachine::default_on_unhandled_rejection);
  2033              addr_of_mut!((*vm).origin_timer).write(std::time::Instant::now());
  2034              addr_of_mut!((*vm).origin_timestamp).write(get_origin_timestamp());
  2035              addr_of_mut!((*vm).smol).write(opts.smol);
  2036              // `Option<{CPU,Heap}ProfilerConfig>` are NOT zero-valid: each
  2037              // payload contains a `bool`, and rustc picks that field's invalid
  2038              // range (not the `&[u8]` null-ptr) as the enum niche, so all-zero
  2039              // bytes decode as `Some` with null-ref slices. Write `None`
  2040              // explicitly.
  2041              addr_of_mut!((*vm).cpu_profiler_config).write(None);
  2042              addr_of_mut!((*vm).heap_profiler_config).write(None);
  2043              // `Option<bool>` uses the bool's invalid range (2) as the niche, so
  2044              // all-zero bytes decode as `Some(false)` — for TLS that would
  2045              // silently disable certificate verification. Write `None` explicitly.
  2046              addr_of_mut!((*vm).default_tls_reject_unauthorized).write(None);
  2047              addr_of_mut!((*vm).ipc).write(None);
  2048              // Non-zero-valid container fields: `Vec`/`Box`/`HashMap`/
  2049              // `ArrayHashMap` all carry a `NonNull` (dangling when empty), and
  2050              // `URL` is a struct of `&[u8]` references — all-zero bytes violate
  2051              // their validity invariants even when len/cap are 0. Write the
  2052              // canonical empty value via `ptr::write` (no Drop of zeroed bytes).
  2053              addr_of_mut!((*vm).preload).write(Vec::new());
  2054              addr_of_mut!((*vm).argv).write(Vec::new());
  2055              addr_of_mut!((*vm).macros).write(Default::default());
  2056              addr_of_mut!((*vm).macro_entry_points).write(Default::default());
  2057              addr_of_mut!((*vm).auto_killer).write(Default::default());
  2058              addr_of_mut!((*vm).commonjs_custom_extensions).write(Default::default());
  2059              addr_of_mut!((*vm).entry_point).write(Default::default());
  2060              addr_of_mut!((*vm).origin).write(Default::default());
  2061              addr_of_mut!((*vm).ref_strings).write(Default::default());
  2062              addr_of_mut!((*vm).modules).write(Default::default());
  2063              addr_of_mut!((*vm).macro_event_loop).write(EventLoop::default());
  2064              addr_of_mut!((*vm).proxy_env_storage).write(Default::default());
  2065              addr_of_mut!((*vm).gc_controller).write(Default::default());
  2066              addr_of_mut!((*vm).channel_ref).write(Default::default());
  2067              addr_of_mut!((*vm).standalone_module_graph).write(opts.graph);
  2068              addr_of_mut!((*vm).initial_script_execution_context_identifier).write(context_id);
  2069              #[cfg(debug_assertions)]
  2070              addr_of_mut!((*vm).debug_thread_id).write(std::thread::current().id());
  2071              // Mutex fields: zeroed atomics ARE valid-unlocked, but write the
  2072              // canonical value so the invariant is explicit.
  2073              addr_of_mut!((*vm).remap_stack_frames_mutex).write(bun_threading::Mutex::new());
  2074              addr_of_mut!((*vm).ref_strings_mutex).write(bun_threading::Mutex::new());
  2075
  2076              addr_of_mut!((*vm).transpiler_store)
  2077                  .write(crate::runtime_transpiler_store::RuntimeTranspilerStore::init());
  2078
  2079              // Event-loop wiring (self-pointers).
  2080              addr_of_mut!((*vm).regular_event_loop).write(EventLoop::default());
  2081              let regular = addr_of_mut!((*vm).regular_event_loop);
  2082              (*regular).virtual_machine = NonNull::new(vm);
  2083              let _ = (*regular).tasks.ensure_unused_capacity(64);
  2084              addr_of_mut!((*vm).event_loop).write(regular);
  2085
  2086              // `source_mappings.map` is a sibling-field backref onto
  2087              // `saved_source_map_table` (spec VirtualMachine.zig:1273).
  2088              addr_of_mut!((*vm).saved_source_map_table)
  2089                  .write(crate::saved_source_map::HashTable::default());
  2090              addr_of_mut!((*vm).source_mappings).write(SavedSourceMap::default());
  2091              (*addr_of_mut!((*vm).source_mappings)).map = addr_of_mut!((*vm).saved_source_map_table);
  2092          }
  2093
  2094          // High-tier per-VM state — Transpiler / Timer::All / entry_point.
  2095          // PORT NOTE (init order): spec VirtualMachine.zig:1241/1259 builds
  2096          // `Transpiler.init` and `.timer = bun.api.Timer.All.init()` as part of
  2097          // the struct initializer BEFORE `JSGlobalObject.create`. The C++ body
  2098          // of `Zig__GlobalObject__create` re-enters via `WTFTimer__create`/
  2099          // `WTFTimer__update` (JSC's GC scheduler), which dereferences
  2100          // `runtime_state().timer` — so this hook MUST run first or that path
  2101          // null-derefs. The post-global tail (`configureDebugger`,
  2102          // `Body.Value.HiveAllocator.init`, spec :1321-1322) is gated TODO in
  2103          // the hook body and will need a separate post-global hook when
  2104          // un-gated. PERF(port): was inline switch.
  2105          if let Some(hooks) = runtime_hooks() {
  2106              // SAFETY: hook contract — `vm` is the unique live VM on this
  2107              // thread. Write through the raw `vm` ptr (not `vm_ref`) so no
  2108              // `&mut VirtualMachine` is held live across the hook call — the
  2109              // hook body itself dereferences `vm`.
  2110              unsafe { (*vm).runtime_state = (hooks.init_runtime_state)(vm, &mut opts) };
  2111          }
  2112
  2113          // JSGlobalObject creation. Spec JSGlobalObject.zig:875 — the wrapper
  2114          // calls `vm.eventLoop().ensureWaker()` before the 5-arg FFI.
  2115          // SAFETY: `vm` is the unique live VM on this thread; raw-ptr deref so
  2116          // no `&mut` is held across the FFI re-entry (`Bun__getVM()` —
  2117          // ZigGlobalObject.cpp:473/961).
  2118          unsafe { (*vm).regular_event_loop.ensure_waker() };
  2119          // `console`/`worker_ptr` are opaque round-trip pointers C++ stores into
  2120          // the new global. `worker_ptr` is the C++ `WebCore::Worker*` (or null on
  2121          // the main thread) — spec VirtualMachine.zig:1477-1484 / JSGlobalObject.zig:876.
  2122          let global = Zig__GlobalObject__create(
  2123              console.cast(),
  2124              context_id,
  2125              opts.mini_mode,
  2126              opts.eval_mode,
  2127              opts.worker_ptr,
  2128          );
  2129          // JSC may mess with the stack size (spec JSGlobalObject.zig:879).
  2130          bun_core::StackCheck::configure_thread();
  2131          // SAFETY: write through the raw `vm` ptr (not `vm_ref`) so no
  2132          // `&mut VirtualMachine` is held live across the FFI call above; same
  2133          // pattern as the `init_runtime_state` hook above. `global` is freshly
  2134          // created and live for VM lifetime; `vm_ptr()` returns the FFI
  2135          // `*mut VM` directly (no `&VM` reborrow), preserving mutable provenance.
  2136          let jsc_vm = unsafe {
  2137              (*vm).global = global;
  2138              (*vm).regular_event_loop.global = NonNull::new(global);
  2139              let jsc_vm = (*global).vm_ptr();
  2140              (*vm).jsc_vm = jsc_vm;
  2141              jsc_vm
  2142          };
  2143          VMHolder::set_cached_global_object(Some(global));
  2144
  2145          // Spec VirtualMachine.zig:1313: `uws.Loop.get().internal_loop_data.jsc_vm
  2146          // = vm.jsc_vm` — must run AFTER `jsc_vm` is set so C/uws callbacks can
  2147          // recover the JSC VM via `internal_loop_data`.
  2148          // SAFETY: `uws::Loop::get()` returns the live per-thread uws loop.
  2149          unsafe {
  2150              (*uws::Loop::get()).internal_loop_data.jsc_vm = jsc_vm.cast();
  2151          }
  2152
  2153          // Spec VirtualMachine.zig:1316 / :1191 — `if (opts.is_main_thread)
  2154          // bun.ParentDeathWatchdog.installOnEventLoop(jsc.EventLoopHandle.init(vm))`.
  2155          // Must run AFTER `ensure_waker()` (above) has set `event_loop_handle`,
  2156          // since on macOS the kqueue registration resolves the platform loop via
  2157          // `event_loop_ctx → uws_loop()`. No-op off macOS / when `--no-orphans`
  2158          // is not enabled. `init_with_module_graph` / `init_bake` route through
  2159          // here with their caller's `is_main_thread`; `init_worker` passes
  2160          // `false` so workers never arm the watchdog (matches spec `initWorker`).
  2161          if opts.is_main_thread {
  2162              bun_io::ParentDeathWatchdog::install_on_event_loop(Self::event_loop_ctx(vm));
  2163          }
  2164
  2165          if opts.smol {
  2166              // SAFETY: written once during init.
  2167              IS_SMOL_MODE.store(true, core::sync::atomic::Ordering::Relaxed);
  2168          }
  2169
  2170          Ok(vm)
  2171      }
  2172
  2173      /// `init` + set `main` to `entry_path`. Port-side convenience for the
  2174      /// `bun -e` / `bun run <file>` boot path; Zig open-codes this in
  2175      /// `run_command.zig`.
  2176      pub fn init_with_main(
  2177          opts: InitOptions,
  2178          entry_path: &[u8],
  2179      ) -> Result<*mut VirtualMachine, bun_core::Error> {
  2180          let vm = Self::init(opts)?;

## vendor/bun/src/jsc/VirtualMachine.rs:2208-2410

- Full-file Git blob: 0ee5e0b90a15caa28dd64a0b4e6d4de998fd91dc
- Full-file SHA-256: aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a
- Full-file bytes: 296637
- Excerpt line span: 2208-2410
- Excerpt SHA-256: 1a51916427c94eead941418530afc276a3b4778c87fec3d7f8a0169f2d7029e3

  2208      /// `eventLoop().waitForPromise(promise)` — spin tick/auto_tick until
  2209      /// `promise` settles. Thin forwarder; body lives in
  2210      /// [`crate::event_loop::EventLoop::wait_for_promise`] (spec event_loop.zig).
  2211      #[inline]
  2212      pub fn wait_for_promise(&mut self, promise: jsc::AnyPromise) {
  2213          // accessed here (no overlapping `&mut EventLoop`).
  2214          self.event_loop_mut().wait_for_promise(promise);
  2215      }
  2216
  2217      /// `eventLoop().autoTick()` — dispatched through the runtime hook
  2218      /// (needs `Timer::All` for the poll timeout).
  2219      #[inline]
  2220      pub fn auto_tick(&mut self) {
  2221          if let Some(hooks) = runtime_hooks() {
  2222              // SAFETY: hook contract — `self` is the live per-thread VM.
  2223              // PERF(port): was inline switch.
  2224              unsafe { (hooks.auto_tick)(self) };
  2225          } else {
  2226              // No high tier (unit tests) — fall back to a non-blocking tick.
  2227              self.event_loop_mut().tick();
  2228          }
  2229      }
  2230
  2231      /// `eventLoop().autoTickActive()` — like [`auto_tick`](Self::auto_tick)
  2232      /// but only sleeps in the uSockets loop while it has active handles
  2233      /// (spec event_loop.zig:456). The real body lives in `event_loop.rs`
  2234      /// behind `` until the b2-cycle (`Timer::All`) breaks; until
  2235      /// then route through the same `auto_tick` hook so drain loops in
  2236      /// `on_before_exit` / `bun_main` still make forward progress.
  2237      #[inline]
  2238      pub fn auto_tick_active(&mut self) {
  2239          if let Some(hooks) = runtime_hooks() {
  2240              // PERF(port): was inline switch — direct call in event_loop.zig.
  2241              // SAFETY: `self` is the live per-thread VM (hook contract).
  2242              unsafe { (hooks.auto_tick_active)(self) };
  2243          } else {
  2244              // No high-tier hook (unit tests) — drain JS tasks only so callers
  2245              // observe forward progress without blocking on the I/O loop.
  2246              self.event_loop_mut().tick();
  2247          }
  2248      }
  2249
  2250      /// `reloadEntryPoint(entry_path)` — set `main`, generate the synthetic
  2251      /// `bun:main` entry, run preloads, and kick off module evaluation.
  2252      pub fn reload_entry_point(
  2253          &mut self,
  2254          entry_path: &[u8],
  2255      ) -> Result<*mut JSInternalPromise, bun_core::Error> {
  2256          self.has_loaded = false;
  2257          self.set_main(entry_path);
  2258          self.main_resolved_path.deref();
  2259          self.main_resolved_path = bun_core::String::empty();
  2260          self.main_hash = bun_watcher::Watcher::get_hash(entry_path);
  2261          self.overridden_main.deinit();
  2262
  2263          let hooks = runtime_hooks();
  2264          let _ = self.ensure_debugger(true);
  2265
  2266          if !self.main_is_html_entrypoint {
  2267              if let Some(hooks) = hooks {
  2268                  let watch = self.is_watcher_enabled();
  2269                  if !(hooks.generate_entry_point)(self, watch, entry_path) {
  2270                      return Err(bun_core::err!("ServerEntryPointGenerate"));
  2271                  }
  2272              }
  2273          }
  2274
  2275          if !self.transpiler.options.disable_transpilation {
  2276              if !self.preload.is_empty() {
  2277                  if let Some(hooks) = hooks {
  2278                      // SAFETY: hook contract.
  2279                      let p = unsafe { (hooks.load_preloads)(self) }?;
  2280                      if !p.is_null() {
  2281                          JSValue::from_cell(p).ensure_still_alive();
  2282                          JSValue::from_cell(p).protect();
  2283                          self.pending_internal_promise = Some(p);
  2284                          self.pending_internal_promise_is_protected = true;
  2285                          return Ok(p);
  2286                      }
  2287                  }
  2288
  2289                  // Check if Module.runMain was patched (spec VirtualMachine.zig:2322-2335).
  2290                  if self.has_patched_run_main {
  2291                      bun_core::hint::cold();
  2292                      self.pending_internal_promise = None;
  2293                      self.pending_internal_promise_is_protected = false;
  2294                      let global_ref = self.global();
  2295                      let argv1 = jsc::bun_string_jsc::create_utf8_for_js(global_ref, MAIN_FILE_NAME)
  2296                          .map_err(|_| bun_core::err!("JSError"))?;
  2297                      let ret = jsc::from_js_host_call_generic(global_ref, || {
  2298                          NodeModuleModule__callOverriddenRunMain(global_ref, argv1)
  2299                      })
  2300                      .map_err(|_| bun_core::err!("JSError"))?;
  2301                      // If the override stored a promise itself, use that; otherwise
  2302                      // wrap its return value.
  2303                      if let Some(stored) = self.pending_internal_promise {
  2304                          return Ok(stored);
  2305                      }
  2306                      let resolved = JSC__JSInternalPromise__resolvedPromise(global_ref, ret);
  2307                      self.pending_internal_promise = Some(resolved);
  2308                      self.pending_internal_promise_is_protected = false;
  2309                      return Ok(resolved);
  2310                  }
  2311              }
  2312
  2313              // PORT NOTE: reshaped for borrowck — capture raw ptr before &self call.
  2314              let global = self.global;
  2315              let global_ref = self.global();
  2316              let promise = if !self.main_is_html_entrypoint {
  2317                  let name = bun_core::String::borrow_utf8(MAIN_FILE_NAME);
  2318                  jsc::JSModuleLoader::load_and_evaluate_module_ptr(global, Some(&name))
  2319                      .map(NonNull::as_ptr)
  2320                      .ok_or_else(|| bun_core::err!("JSError"))?
  2321              } else {
  2322                  let p: *mut JSInternalPromise = jsc::from_js_host_call_generic(global_ref, || {
  2323                      Bun__loadHTMLEntryPoint(global_ref)
  2324                  })
  2325                  .map_err(|_| bun_core::err!("JSError"))?;
  2326                  if p.is_null() {
  2327                      return Err(bun_core::err!("JSError"));
  2328                  }
  2329                  p
  2330              };
  2331
  2332              self.pending_internal_promise = Some(promise);
  2333              self.pending_internal_promise_is_protected = false;
  2334              JSValue::from_cell(promise).ensure_still_alive();
  2335              Ok(promise)
  2336          } else {
  2337              let global = self.global;
  2338              let main_str = bun_core::String::from_bytes(self.main());
  2339              let promise =
  2340                  jsc::JSModuleLoader::load_and_evaluate_module_ptr(global, Some(&main_str))
  2341                      .map(NonNull::as_ptr)
  2342                      .ok_or_else(|| bun_core::err!("JSError"))?;
  2343              self.pending_internal_promise = Some(promise);
  2344              self.pending_internal_promise_is_protected = false;
  2345              JSValue::from_cell(promise).ensure_still_alive();
  2346              Ok(promise)
  2347          }
  2348      }
  2349
  2350      /// `loadEntryPoint(entry_path)` — `reload_entry_point` + spin until the
  2351      /// returned promise settles.
  2352      pub fn load_entry_point(
  2353          &mut self,
  2354          entry_path: &[u8],
  2355      ) -> Result<*mut JSInternalPromise, bun_core::Error> {
  2356          let promise = self.reload_entry_point(entry_path)?;
  2357
  2358          // pending_internal_promise can change if hot module reloading is enabled
  2359          if self.is_watcher_enabled() {
  2360              // accessed here (no overlapping `&mut EventLoop`).
  2361              self.event_loop_mut().perform_gc();
  2362              loop {
  2363                  let Some(p) = self.pending_internal_promise else {
  2364                      break;
  2365                  };
  2366                  // SAFETY: `p` is a live JSC heap cell tracked by the VM.
  2367                  if crate::JSPromise::status_ptr(p) != crate::js_promise::Status::Pending {
  2368                      break;
  2369                  }
  2370                  self.event_loop_mut().tick();
  2371                  let Some(p) = self.pending_internal_promise else {
  2372                      break;
  2373                  };
  2374                  // SAFETY: see above.
  2375                  if crate::JSPromise::status_ptr(p) == crate::js_promise::Status::Pending {
  2376                      self.auto_tick();
  2377                  }
  2378              }
  2379          } else {
  2380              // SAFETY: `promise` is a live JSC heap cell.
  2381              if crate::JSPromise::status_ptr(promise) == crate::js_promise::Status::Rejected {
  2382                  return Ok(promise);
  2383              }
  2384              self.event_loop_mut().perform_gc();
  2385              self.wait_for_promise(jsc::AnyPromise::Internal(promise));
  2386          }
  2387
  2388          Ok(self.pending_internal_promise.unwrap_or(promise))
  2389      }
  2390
  2391      /// Drain pending tasks/microtasks if the event loop is not currently
  2392      /// re-entered. Port-side convenience used after top-level evaluation on
  2393      /// the `bun -e` path (Zig open-codes `eventLoop().tick()` +
  2394      /// `drainMicrotasks()` at each call site).
  2395      pub fn drain_queues_if_needed(&mut self) {
  2396          // SAFETY: `event_loop` is a self-pointer into this VM; uniquely
  2397          // accessed here (no overlapping `&mut EventLoop`).
  2398          if self.event_loop_mut().entered_event_loop_count > 0 {
  2399              return;
  2400          }
  2401          self.event_loop_mut().tick();
  2402          let _ = self.event_loop_mut().drain_microtasks();
  2403          self.global().handle_rejected_promises();
  2404      }
  2405  }
  2406
  2407  /// Spec VirtualMachine.zig:2032 `processFetchLog`. Synthesize a JS
  2408  /// `BuildMessage` / `ResolveMessage` / `AggregateError` from the parser
  2409  /// `log` and write it into `ret` as `.err(..)` so the C++ module-loader
  2410  /// (`Bun__onFulfillAsyncModule`, ModuleLoader.cpp) rejects the import promise

## vendor/bun/src/jsc/VirtualMachine.rs:4302-4360

- Full-file Git blob: 0ee5e0b90a15caa28dd64a0b4e6d4de998fd91dc
- Full-file SHA-256: aeb953a4b048201059b7230b0f2d64c8a145ca196f011f3a93993240607a9c3a
- Full-file bytes: 296637
- Excerpt line span: 4302-4360
- Excerpt SHA-256: 963d06b8554567c2fe0c8573723e1661c3cd358bfa841c0a00883886b724c95d

  4302      pub fn destroy(&mut self) {
  4303          // PORT NOTE: Zig `auto_killer.deinit()` — `ProcessAutoKiller`'s `Drop`
  4304          // is the deinit body; take()+drop runs it without dropping `self`.
  4305          drop(core::mem::take(&mut self.auto_killer));
  4306
  4307          // PORT NOTE: Zig frees the thread-local `source_code_printer` static
  4308          // in `deinit`; here it's `SOURCE_CODE_PRINTER` (boxed via
  4309          // `ensure_source_code_printer`).
  4310          if let Some(printer) = SOURCE_CODE_PRINTER.take() {
  4311              // SAFETY: `printer` was produced by `heap::alloc` in
  4312              // `ensure_source_code_printer` and is exclusively owned by this
  4313              // thread's VM.
  4314              drop(unsafe { bun_core::heap::take(printer.as_ptr()) });
  4315          }
  4316
  4317          // PORT NOTE: `SavedSourceMap`'s `Drop` is the Zig `deinit()`; it frees
  4318          // each stored map and `deinit()`s the sibling `saved_source_map_table`.
  4319          drop(core::mem::take(&mut self.source_mappings));
  4320
  4321          if let Some(rare) = self.rare_data.take() {
  4322              if let Some(hooks) = runtime_hooks() {
  4323                  (hooks.cron_clear_all_teardown)(self);
  4324              }
  4325              // Paired with `rare_data()`'s register_root_region. Without this,
  4326              // every terminated Worker leaves a stale LSAN root entry pointing
  4327              // into a freed arena.
  4328              bun_core::asan::unregister_root_region(
  4329                  core::ptr::from_ref::<RareData>(&*rare).cast(),
  4330                  core::mem::size_of::<RareData>(),
  4331              );
  4332              drop(rare);
  4333          }
  4334
  4335          // PORT NOTE: Zig `proxy_env_storage.deinit()` — drops all `Arc`-held
  4336          // proxy strings; `ProxyEnvStorage: Default` so take()+drop suffices.
  4337          drop(core::mem::take(&mut self.proxy_env_storage));
  4338          self.overridden_main.deinit();
  4339
  4340          // PORT NOTE: Zig frees `timer`/`entry_point` as value fields of `self`;
  4341          // here they live in the high-tier `RuntimeState` box, so dispatch the
  4342          // reclaim through the hook. PERF(port): was inline switch.
  4343          if let Some(hooks) = runtime_hooks() {
  4344              let state = core::mem::replace(&mut self.runtime_state, core::ptr::null_mut());
  4345              // SAFETY: hook contract — `state` is exactly the pointer
  4346              // `init_runtime_state` returned for this VM (or null), handed back
  4347              // once on the same thread; `self` is the live per-thread VM.
  4348              unsafe { (hooks.deinit_runtime_state)(std::ptr::from_mut(self), state) };
  4349          }
  4350          self.has_terminated = true;
  4351      }
  4352      /// Spec VirtualMachine.zig:2134 `printException`.
  4353      ///
  4354      /// PORT NOTE: Zig is `comptime Writer`-generic; collapse to the concrete
  4355      /// `bun_core::io::Writer` since every call site passes
  4356      /// `Output.errorWriterBuffered()`.
  4357      pub fn print_exception(
  4358          &mut self,
  4359          exception: &Exception,
  4360          exception_list: Option<&mut ExceptionList>,

## vendor/bun/src/jsc/VM.rs:1-220

- Full-file Git blob: 47398ab74d014061a65b17fa7201d85ff4489778
- Full-file SHA-256: 47aa4f3b8642dfefe21dd56c2fa446889cd701cc3e9fdbf7d962bf4f6aee7119
- Full-file bytes: 8822
- Excerpt line span: 1-220
- Excerpt SHA-256: 280df3325eadebf30eae1d83123882dc229d832f74e57f7da99ac8bb895a76f6

     1  use core::cell::UnsafeCell;
     2  use core::ffi::c_void;
     3  use core::marker::{PhantomData, PhantomPinned};
     4
     5  use crate::{Exception, ExceptionValidationScope, JSGlobalObject, JSValue, JsError};
     6
     7  // TODO(port): move to <jsc>_sys
     8  //
     9  // All JSC__VM__* shims take only a `JSC::VM*` (and at most a
    10  // `JSGlobalObject*` / `JSC::Exception*` / scalar). `VM` and `JSGlobalObject`
    11  // are opaque `UnsafeCell`-backed ZST handles, so `&VM` is ABI-identical to a
    12  // non-null `VM*` and the C++ side mutating through it does not violate Rust
    13  // aliasing (interior mutability; zero Rust-visible bytes). Declaring the
    14  // params as references and the fns as `safe fn` moves the validity proof into
    15  // the type signature and removes the per-call-site `unsafe { }` wrappers.
    16  // `holdAPILock` keeps a raw `*mut c_void` ctx (opaque round-trip; C++ never
    17  // dereferences it as Rust data) so it stays `unsafe fn`.
    18  unsafe extern "C" {
    19      safe fn JSC__VM__deinit(vm: &VM, global_object: &JSGlobalObject);
    20      safe fn JSC__VM__setControlFlowProfiler(vm: &VM, enabled: bool);
    21      safe fn JSC__VM__hasExecutionTimeLimit(vm: &VM) -> bool;
    22      // safe: `VM` is an opaque `UnsafeCell`-backed ZST handle (`&` is ABI-identical
    23      // to non-null `*const`); `ctx` is an opaque round-trip pointer C++ only forwards
    24      // to `callback` (never dereferenced as Rust data) — same contract as
    25      // `JSC__JSGlobalObject__queueMicrotaskCallback`.
    26      safe fn JSC__VM__holdAPILock(
    27          this: &VM,
    28          ctx: *mut c_void,
    29          callback: extern "C" fn(ctx: *mut c_void),
    30      );
    31      safe fn JSC__VM__getAPILock(vm: &VM);
    32      safe fn JSC__VM__releaseAPILock(vm: &VM);
    33      safe fn JSC__VM__reportExtraMemory(vm: &VM, size: usize);
    34      safe fn JSC__VM__deleteAllCode(vm: &VM, global_object: &JSGlobalObject);
    35      safe fn JSC__VM__shrinkFootprint(vm: &VM);
    36      safe fn JSC__VM__runGC(vm: &VM, sync: bool) -> usize;
    37      safe fn JSC__VM__heapSize(vm: &VM) -> usize;
    38      safe fn JSC__VM__collectAsync(vm: &VM);
    39      safe fn JSC__VM__setExecutionForbidden(vm: &VM, forbidden: bool);
    40      safe fn JSC__VM__setExecutionTimeLimit(vm: &VM, timeout: f64);
    41      safe fn JSC__VM__clearExecutionTimeLimit(vm: &VM);
    42      safe fn JSC__VM__executionForbidden(vm: &VM) -> bool;
    43      safe fn JSC__VM__notifyNeedTermination(vm: &VM);
    44      safe fn JSC__VM__notifyNeedWatchdogCheck(vm: &VM);
    45      safe fn JSC__VM__notifyNeedDebuggerBreak(vm: &VM);
    46      safe fn JSC__VM__notifyNeedShellTimeoutCheck(vm: &VM);
    47      safe fn JSC__VM__isEntered(vm: &VM) -> bool;
    48      safe fn JSC__VM__throwError(vm: &VM, global_object: &JSGlobalObject, value: JSValue);
    49      safe fn JSC__VM__releaseWeakRefs(vm: &VM);
    50      safe fn JSC__VM__drainMicrotasks(vm: &VM);
    51      safe fn JSC__VM__externalMemorySize(vm: &VM) -> usize;
    52      safe fn JSC__VM__blockBytesAllocated(vm: &VM) -> usize;
    53      safe fn JSC__VM__performOpportunisticallyScheduledTasks(vm: &VM, until: f64);
    54  }
    55
    56  bun_opaque::opaque_ffi! {
    57      /// Opaque handle to a `JSC::VM`.
    58      pub struct VM;
    59  }
    60
    61  #[repr(u8)]
    62  #[derive(Copy, Clone, Eq, PartialEq)]
    63  pub enum HeapType {
    64      SmallHeap = 0,
    65      LargeHeap = 1,
    66  }
    67
    68  impl VM {
    69      // PORT NOTE: `JSC__VM__create` was removed from bindings.cpp (Bun creates
    70      // its VM via `Zig::GlobalObject::create` → `WebWorker__createVM` instead).
    71      // The Zig `VM.create` wrapper is dead code; do not port it.
    72
    73      // PORT NOTE: not `impl Drop` — takes a `global_object` param and `VM` is an opaque FFI handle.
    74      pub fn deinit(&self, global_object: &JSGlobalObject) {
    75          JSC__VM__deinit(self, global_object)
    76      }
    77
    78      pub fn set_control_flow_profiler(&self, enabled: bool) {
    79          JSC__VM__setControlFlowProfiler(self, enabled)
    80      }
    81
    82      pub fn is_jit_enabled() -> bool {
    83          crate::cpp::JSC__VM__isJITEnabled()
    84      }
    85
    86      pub fn has_execution_time_limit(&self) -> bool {
    87          JSC__VM__hasExecutionTimeLimit(self)
    88      }
    89
    90      /// deprecated in favor of `get_api_lock` to avoid an annoying callback wrapper
    91      #[deprecated = "use get_api_lock"]
    92      pub fn hold_api_lock(&self, ctx: *mut c_void, callback: extern "C" fn(ctx: *mut c_void)) {
    93          JSC__VM__holdAPILock(self, ctx, callback)
    94      }
    95
    96      /// See `JSLock.h` in WebKit for more detail on how the API lock prevents races.
    97      pub fn get_api_lock(&self) -> Lock<'_> {
    98          JSC__VM__getAPILock(self);
    99          Lock { vm: self }
   100      }
   101
   102      // PORT NOTE: `JSC__VM__deferGC` was removed from bindings.cpp in the
   103      // WebKit-bump that introduced `JSC::DeferGC` RAII; the Zig `deferGC`
   104      // wrapper is dead code. Callers should use `holdAPILock`/`DeferGC` on the
   105      // C++ side instead.
   106
   107      pub fn report_extra_memory(&self, size: usize) {
   108          crate::mark_binding!();
   109          JSC__VM__reportExtraMemory(self, size)
   110      }
   111
   112      /// Alias retained for parity with the Zig comment naming this the
   113      /// "deprecated" GC accounting hook (the underlying C++ is
   114      /// `Heap::deprecatedReportExtraMemory`). Forward to [`report_extra_memory`].
   115      #[inline]
   116      pub fn deprecated_report_extra_memory(&self, size: usize) {
   117          self.report_extra_memory(size);
   118      }
   119
   120      pub fn delete_all_code(&self, global_object: &JSGlobalObject) {
   121          JSC__VM__deleteAllCode(self, global_object)
   122      }
   123
   124      pub fn shrink_footprint(&self) {
   125          JSC__VM__shrinkFootprint(self)
   126      }
   127
   128      pub fn run_gc(&self, sync: bool) -> usize {
   129          JSC__VM__runGC(self, sync)
   130      }
   131
   132      pub fn heap_size(&self) -> usize {
   133          JSC__VM__heapSize(self)
   134      }
   135
   136      pub fn collect_async(&self) {
   137          JSC__VM__collectAsync(self)
   138      }
   139
   140      pub fn set_execution_forbidden(&self, forbidden: bool) {
   141          JSC__VM__setExecutionForbidden(self, forbidden)
   142      }
   143
   144      pub fn set_execution_time_limit(&self, timeout: f64) {
   145          JSC__VM__setExecutionTimeLimit(self, timeout)
   146      }
   147
   148      pub fn clear_execution_time_limit(&self) {
   149          JSC__VM__clearExecutionTimeLimit(self)
   150      }
   151
   152      pub fn execution_forbidden(&self) -> bool {
   153          JSC__VM__executionForbidden(self)
   154      }
   155
   156      // These four functions fire VM traps. To understand what that means, see VMTraps.h for a giant explainer.
   157      // These may be called concurrently from another thread.
   158
   159      /// Fires NeedTermination Trap. Thread safe. See jsc's "VMTraps.h" for explaination on traps.
   160      pub fn notify_need_termination(&self) {
   161          JSC__VM__notifyNeedTermination(self)
   162      }
   163
   164      /// Fires NeedWatchdogCheck Trap. Thread safe. See jsc's "VMTraps.h" for explaination on traps.
   165      pub fn notify_need_watchdog_check(&self) {
   166          JSC__VM__notifyNeedWatchdogCheck(self)
   167      }
   168
   169      /// Fires NeedDebuggerBreak Trap. Thread safe. See jsc's "VMTraps.h" for explaination on traps.
   170      pub fn notify_need_debugger_break(&self) {
   171          JSC__VM__notifyNeedDebuggerBreak(self)
   172      }
   173
   174      /// Fires NeedShellTimeoutCheck Trap. Thread safe. See jsc's "VMTraps.h" for explaination on traps.
   175      pub fn notify_need_shell_timeout_check(&self) {
   176          JSC__VM__notifyNeedShellTimeoutCheck(self)
   177      }
   178
   179      pub fn is_entered(&self) -> bool {
   180          JSC__VM__isEntered(self)
   181      }
   182
   183      pub fn is_termination_exception(&self, exception: &Exception) -> bool {
   184          crate::cpp::JSC__VM__isTerminationException(self, exception)
   185      }
   186
   187      pub fn has_termination_request(&self) -> bool {
   188          crate::cpp::JSC__VM__hasTerminationRequest(self)
   189      }
   190
   191      pub fn clear_has_termination_request(&self) {
   192          crate::cpp::JSC__VM__clearHasTerminationRequest(self)
   193      }
   194
   195      #[track_caller]
   196      pub fn throw_error(&self, global_object: &JSGlobalObject, value: JSValue) -> JsError {
   197          crate::validation_scope!(scope, global_object);
   198          scope.assert_no_exception();
   199          JSC__VM__throwError(self, global_object, value);
   200          scope.assert_exception_presence_matches(true);
   201          JsError::Thrown
   202      }
   203
   204      pub fn release_weak_refs(&self) {
   205          JSC__VM__releaseWeakRefs(self)
   206      }
   207
   208      pub fn drain_microtasks(&self) {
   209          JSC__VM__drainMicrotasks(self)
   210      }
   211
   212      pub fn external_memory_size(&self) -> usize {
   213          JSC__VM__externalMemorySize(self)
   214      }
   215
   216      /// `RESOURCE_USAGE` build option in JavaScriptCore is required for this function
   217      /// This is faster than checking the heap size
   218      pub fn block_bytes_allocated(&self) -> usize {
   219          JSC__VM__blockBytesAllocated(self)
   220      }

## vendor/bun/src/jsc/JSGlobalObject.rs:190-235

- Full-file Git blob: bc6ab6214c903a2467e2f60bdf9ddea2b7f35fcf
- Full-file SHA-256: d6dc512fb3021f0c57bee74fe66230eaebf5f7b94f3bf75602af0ea3a045786e
- Full-file bytes: 69350
- Excerpt line span: 190-235
- Excerpt SHA-256: 54c2537ad8324a0ba74c344f42377bacf79e67147fe19a60bcb3cc9e3ae0f6e4

   190                  &raw mut dt.minute,
   191                  &raw mut dt.second,
   192                  &raw mut dt.weekday,
   193              );
   194          }
   195          dt
   196      }
   197
   198      pub fn throw_todo(&self, msg: &[u8]) -> JsError {
   199          let err = self.create_error_instance(format_args!("{}", bstr::BStr::new(msg)));
   200          if err.is_empty() {
   201              debug_assert!(self.has_exception());
   202              return JsError::Thrown;
   203          }
   204          let name_value = match BunString::static_str("TODOError").to_js(self) {
   205              Ok(v) => v,
   206              Err(_) => return JsError::Thrown,
   207          };
   208          err.put(self, b"name", name_value);
   209          self.throw_value(err)
   210      }
   211
   212      #[inline]
   213      pub fn request_termination(&self) {
   214          JSGlobalObject__requestTermination(self)
   215      }
   216
   217      #[inline]
   218      pub fn clear_termination_exception(&self) {
   219          JSGlobalObject__clearTerminationException(self)
   220      }
   221
   222      pub fn set_time_zone(&self, time_zone: &ZigString) -> bool {
   223          JSGlobalObject__setTimeZone(self, time_zone)
   224      }
   225
   226      #[inline]
   227      pub fn to_js_value(&self) -> JSValue {
   228          // JSValue is #[repr(transparent)] over the encoded pointer-width word; encoding a
   229          // cell pointer is exactly Zig's `@enumFromInt(@intFromPtr(globalThis))`.
   230          JSValue::from_encoded(std::ptr::from_ref::<Self>(self) as usize)
   231      }
   232
   233      pub fn throw_invalid_arguments(&self, args: Arguments<'_>) -> JsError {
   234          let err = self.to_invalid_arguments(args);
   235          self.throw_value(err)

## vendor/bun/src/jsc/JSGlobalObject.rs:960-1005

- Full-file Git blob: bc6ab6214c903a2467e2f60bdf9ddea2b7f35fcf
- Full-file SHA-256: d6dc512fb3021f0c57bee74fe66230eaebf5f7b94f3bf75602af0ea3a045786e
- Full-file bytes: 69350
- Excerpt line span: 960-1005
- Excerpt SHA-256: 0c9d81490157f52adaf8b0a9638043c917728f762aeace0a9bf21cf9891532d9

   960      /// (`JSGlobalObject__hasException`) constructs a temporary `TopExceptionScope`, whose
   961      /// ctor *does* call `verifyExceptionCheckNeedIsSatisfied` — so this asserts if
   962      /// `vm.m_needExceptionCheck` was left set by a prior un-scoped FFI call. The remaining
   963      /// call sites in the port (1:1 with the `.zig` spec) follow `JsResult`-returning helpers
   964      /// that already opened a scope and cleared the bit, so they are sound. New code must not
   965      /// pair this with a raw `extern "C"` throwing call — use the generated
   966      /// [`crate::cpp`] wrappers or [`top_scope!`](crate::top_scope) instead.
   967      pub fn has_exception(&self) -> bool {
   968          JSGlobalObject__hasException(self)
   969      }
   970
   971      pub fn clear_exception(&self) {
   972          JSGlobalObject__clearException(self)
   973      }
   974
   975      /// Clear the currently active exception off the VM unless it is a
   976      /// termination exception.
   977      ///
   978      /// Returns `true` if the exception was cleared, `false` if it was a
   979      /// termination exception. Use `clear_exception` to unconditionally clear
   980      /// exceptions.
   981      ///
   982      /// It is safe to call this function when no exception is present.
   983      pub fn clear_exception_except_termination(&self) -> bool {
   984          JSGlobalObject__clearExceptionExceptTermination(self)
   985      }
   986
   987      /// Clears the current exception and returns that value. Requires compile-time
   988      /// proof of an exception via `JsError`.
   989      pub fn take_exception(&self, proof: JsError) -> JSValue {
   990          match proof {
   991              JsError::Thrown => {}
   992              JsError::OutOfMemory => {
   993                  let _ = self.throw_out_of_memory();
   994              }
   995              JsError::Terminated => {}
   996          }
   997
   998          self.try_take_exception().unwrap_or_else(|| {
   999              panic!(
  1000                  "A JavaScript exception was thrown, but it was cleared before it could be read."
  1001              );
  1002          })
  1003      }
  1004
  1005      pub fn take_error(&self, proof: JsError) -> JSValue {

## vendor/bun/src/jsc/bindings/bindings.cpp:4880-4995

- Full-file Git blob: b08737cc9ba97c08fccd8d291f73cc03275031d0
- Full-file SHA-256: e32cd326cc1592bed6f70bff8eba95bfd855a17a6998567976b352e9478d5bff
- Full-file bytes: 255398
- Excerpt line span: 4880-4995
- Excerpt SHA-256: 0354fb569edc2eb9f9c3b6b2a9a97285f12821a9b67e180e01fb5416c8c2cfcb

  4880  }
  4881
  4882  bool JSC__JSValue__isTerminationException(JSC::EncodedJSValue JSValue0)
  4883  {
  4884      JSC::Exception* exception = dynamicDowncast<JSC::Exception>(JSC::JSValue::decode(JSValue0));
  4885      if (exception == nullptr)
  4886          return false;
  4887
  4888      return exception->vm().isTerminationException(exception);
  4889  }
  4890
  4891  void JSC__VM__shrinkFootprint(JSC::VM* arg0)
  4892  {
  4893      arg0->shrinkFootprintWhenIdle();
  4894  };
  4895
  4896  void JSC__VM__holdAPILock(JSC::VM* arg0, void* ctx, void (*callback)(void* arg0))
  4897  {
  4898      JSC::JSLockHolder locker(arg0);
  4899      callback(ctx);
  4900  }
  4901
  4902  // The following two functions are copied 1:1 from JSLockHolder to provide a
  4903  // new, more ergonomic binding for interacting with the lock from Zig
  4904  // https://github.com/WebKit/WebKit/blob/main/Source/JavaScriptCore/runtime/JSLock.cpp
  4905
  4906  extern "C" void JSC__VM__getAPILock(JSC::VM* vm)
  4907  {
  4908      // https://github.com/WebKit/WebKit/blob/6cb5017d237ef7cb898582a22f05acca22322845/Source/JavaScriptCore/runtime/JSLock.cpp#L67
  4909      vm->apiLock().lock();
  4910  }
  4911
  4912  extern "C" void JSC__VM__releaseAPILock(JSC::VM* vm)
  4913  {
  4914      // https://github.com/WebKit/WebKit/blob/6cb5017d237ef7cb898582a22f05acca22322845/Source/JavaScriptCore/runtime/JSLock.cpp#L72
  4915      RefPtr<JSLock> apiLock(&vm->apiLock());
  4916      apiLock->unlock();
  4917  }
  4918
  4919  void JSC__JSString__iterator(JSC::JSString* arg0, JSC::JSGlobalObject* arg1, void* arg2)
  4920  {
  4921      jsstring_iterator* iter = (jsstring_iterator*)arg2;
  4922      arg0->value(iter);
  4923  }
  4924
  4925  void JSC__VM__deleteAllCode(JSC::VM* arg1, JSC::JSGlobalObject* globalObject)
  4926  {
  4927      JSC::JSLockHolder locker(globalObject->vm());
  4928
  4929      arg1->drainMicrotasks();
  4930      globalObject->moduleLoader()->clearAll();
  4931      arg1->deleteAllCode(JSC::DeleteAllCodeEffort::PreventCollectionAndDeleteAllCode);
  4932      arg1->heap.reportAbandonedObjectGraph();
  4933  }
  4934
  4935  void JSC__VM__reportExtraMemory(JSC::VM* arg0, size_t arg1)
  4936  {
  4937      arg0->heap.deprecatedReportExtraMemory(arg1);
  4938  }
  4939
  4940  void JSC__VM__deinit(JSC::VM* arg1, JSC::JSGlobalObject* globalObject)
  4941  {
  4942  }
  4943
  4944  void JSC__VM__drainMicrotasks(JSC::VM* arg0)
  4945  {
  4946      arg0->drainMicrotasks();
  4947  }
  4948
  4949  bool JSC__VM__executionForbidden(JSC::VM* arg0)
  4950  {
  4951      return (*arg0).executionForbidden();
  4952  }
  4953
  4954  bool JSC__VM__isEntered(JSC::VM* arg0)
  4955  {
  4956      return (*arg0).isEntered();
  4957  }
  4958
  4959  [[ZIG_EXPORT(nothrow)]]
  4960  bool JSC__VM__isTerminationException(JSC::VM* vm, JSC::Exception* exception)
  4961  {
  4962      return vm->isTerminationException(exception);
  4963  }
  4964
  4965  [[ZIG_EXPORT(nothrow)]]
  4966  void JSC__VM__clearHasTerminationRequest(JSC::VM* vm)
  4967  {
  4968      vm->clearHasTerminationRequest();
  4969  }
  4970  [[ZIG_EXPORT(nothrow)]]
  4971  bool JSC__VM__hasTerminationRequest(JSC::VM* vm)
  4972  {
  4973      return vm->hasTerminationRequest();
  4974  }
  4975
  4976  void JSC__VM__setExecutionForbidden(JSC::VM* arg0, bool arg1)
  4977  {
  4978      (*arg0).setExecutionForbidden();
  4979  }
  4980
  4981  // These may be called concurrently from another thread.
  4982  void JSC__VM__notifyNeedTermination(JSC::VM* arg0)
  4983  {
  4984      JSC::VM& vm = *arg0;
  4985      bool didEnter = vm.currentThreadIsHoldingAPILock();
  4986      if (didEnter)
  4987          vm.apiLock().unlock();
  4988      vm.notifyNeedTermination();
  4989      if (didEnter)
  4990          vm.apiLock().lock();
  4991  }
  4992  void JSC__VM__notifyNeedDebuggerBreak(JSC::VM* arg0)
  4993  {
  4994      (*arg0).notifyNeedDebuggerBreak();
  4995  }

## vendor/bun/src/jsc/bindings/bindings.cpp:6124-6145

- Full-file Git blob: b08737cc9ba97c08fccd8d291f73cc03275031d0
- Full-file SHA-256: e32cd326cc1592bed6f70bff8eba95bfd855a17a6998567976b352e9478d5bff
- Full-file bytes: 255398
- Excerpt line span: 6124-6145
- Excerpt SHA-256: 7b287faffbfca5144f2a2a0edffa57213c3be262a67697e550093b23efda32cb

  6124  extern "C" bool JSGlobalObject__hasException(JSC::JSGlobalObject* globalObject)
  6125  {
  6126      return DECLARE_TOP_EXCEPTION_SCOPE(globalObject->vm()).exception() != 0;
  6127  }
  6128
  6129  extern "C" void JSGlobalObject__clearException(JSC::JSGlobalObject* globalObject)
  6130  {
  6131      (void)DECLARE_TOP_EXCEPTION_SCOPE(globalObject->vm()).tryClearException();
  6132  }
  6133
  6134  extern "C" bool JSGlobalObject__clearExceptionExceptTermination(JSC::JSGlobalObject* globalObject)
  6135  {
  6136      return DECLARE_TOP_EXCEPTION_SCOPE(globalObject->vm()).clearExceptionExceptTermination();
  6137  }
  6138
  6139  extern "C" JSC::EncodedJSValue JSGlobalObject__tryTakeException(JSC::JSGlobalObject* globalObject)
  6140  {
  6141      auto scope = DECLARE_TOP_EXCEPTION_SCOPE(globalObject->vm());
  6142
  6143      if (auto exception = scope.exception()) {
  6144          (void)scope.tryClearException();
  6145          return JSC::JSValue::encode(exception);

## vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp:2988-3055

- Full-file Git blob: a7969dc80971bbd085baaa8178a9e0a1a765cfa1
- Full-file SHA-256: 72f4a089c9e75ad919be6ebe8d47e846db74b1872aecdad7f9b217e3c696cf9d
- Full-file bytes: 185486
- Excerpt line span: 2988-3055
- Excerpt SHA-256: bf244947a76a560b2e3623e0277ee02fa45543a8d3d043f3fa3bb0fe9079760a

  2988  }
  2989
  2990  extern "C" [[ZIG_EXPORT(nothrow)]] void JSC__JSGlobalObject__addGc(JSC::JSGlobalObject* globalObject)
  2991  {
  2992      auto& vm = JSC::getVM(globalObject);
  2993      globalObject->putDirectNativeFunction(vm, globalObject, JSC::Identifier::fromString(vm, "gc"_s), 0, functionJsGc, ImplementationVisibility::Public, JSC::NoIntrinsic, PropertyAttribute::DontEnum | 0);
  2994  }
  2995
  2996  // ====================== end conditional builtin globals ======================
  2997
  2998  uint8_t GlobalObject::drainMicrotasks()
  2999  {
  3000      auto& vm = this->vm();
  3001      auto scope = DECLARE_TOP_EXCEPTION_SCOPE(vm);
  3002
  3003      if (auto* exception = scope.exception()) [[unlikely]] {
  3004          if (vm.isTerminationException(exception)) [[unlikely]] {
  3005              return 1;
  3006          }
  3007
  3008  #if ASSERT_ENABLED
  3009          (void)scope.tryClearException();
  3010          // We should not have an exception here.
  3011          // But it's an easy mistake to make.
  3012          // Let's log it so that we can debug this.
  3013          Bun__reportError(this, JSValue::encode(exception));
  3014
  3015          // And re-throw it to preserve the production behavior.
  3016          auto throwScope = DECLARE_THROW_SCOPE(vm);
  3017          throwScope.throwException(this, exception);
  3018          throwScope.release();
  3019  #endif
  3020      }
  3021      scope.assertNoExceptionExceptTermination();
  3022
  3023      if (auto nextTickQueue = this->m_nextTickQueue.get()) {
  3024          nextTickQueue->drain(vm, this);
  3025          if (auto* exception = scope.exception()) {
  3026              if (vm.isTerminationException(exception)) {
  3027                  return 1;
  3028              }
  3029              (void)scope.tryClearException();
  3030              this->reportUncaughtExceptionAtEventLoop(this, exception);
  3031              return 0;
  3032          }
  3033      }
  3034      vm.drainMicrotasks();
  3035      if (auto* exception = scope.exception()) {
  3036          if (vm.isTerminationException(exception)) {
  3037              return 1;
  3038          }
  3039          (void)scope.tryClearException();
  3040          this->reportUncaughtExceptionAtEventLoop(this, exception);
  3041      }
  3042
  3043      return 0;
  3044  }
  3045
  3046  extern "C" uint8_t JSC__JSGlobalObject__drainMicrotasks(Zig::GlobalObject* globalObject)
  3047  {
  3048      return globalObject->drainMicrotasks();
  3049  }
  3050
  3051  extern "C" EncodedJSValue JSC__JSGlobalObject__getHTTP2CommonString(Zig::GlobalObject* globalObject, uint32_t hpack_index)
  3052  {
  3053      auto value = globalObject->http2CommonStrings().getStringFromHPackIndex(hpack_index, globalObject);
  3054      if (value != nullptr) {
  3055          return JSValue::encode(value);

## vendor/bun/src/jsc/bindings/ZigGlobalObject.cpp:3120-3155

- Full-file Git blob: a7969dc80971bbd085baaa8178a9e0a1a765cfa1
- Full-file SHA-256: 72f4a089c9e75ad919be6ebe8d47e846db74b1872aecdad7f9b217e3c696cf9d
- Full-file bytes: 185486
- Excerpt line span: 3120-3155
- Excerpt SHA-256: 7e691b23fcbc2c37aeb2166c9429c228ef86eba4774261fbd80a6e9e738cbf49

  3120
  3121      if (WTF::setTimeZoneOverride(Zig::toString(*timeZone))) {
  3122          vm.dateCache.resetIfNecessarySlow();
  3123          return true;
  3124      }
  3125
  3126      return false;
  3127  }
  3128
  3129  extern "C" void JSGlobalObject__requestTermination(JSC::JSGlobalObject* globalObject)
  3130  {
  3131      auto& vm = JSC::getVM(globalObject);
  3132      vm.ensureTerminationException();
  3133      vm.setHasTerminationRequest();
  3134  }
  3135
  3136  extern "C" void JSGlobalObject__clearTerminationException(JSC::JSGlobalObject* globalObject)
  3137  {
  3138      auto& vm = JSC::getVM(globalObject);
  3139      // Clear the request for the termination exception to be thrown
  3140      vm.clearHasTerminationRequest();
  3141      // In case it actually has been thrown, clear the exception itself as well.
  3142      // tryClearException() refuses to clear termination exceptions, so use
  3143      // TopExceptionScope::clearException() which clears unconditionally —
  3144      // this function's whole purpose is to clear that specific exception so
  3145      // execution can resume (e.g. for process.on('exit') after terminate()).
  3146      auto scope = DECLARE_TOP_EXCEPTION_SCOPE(vm);
  3147      if (scope.exception() && vm.isTerminationException(scope.exception())) {
  3148          scope.clearException();
  3149      }
  3150  }
  3151
  3152  extern "C" void Bun__queueTask(JSC::JSGlobalObject*, WebCore::EventLoopTask* task);
  3153  extern "C" void Bun__queueTaskConcurrently(JSC::JSGlobalObject*, WebCore::EventLoopTask* task);
  3154  extern "C" [[ZIG_EXPORT(check_slow)]] void Bun__performTask(Zig::GlobalObject* globalObject, WebCore::EventLoopTask* task)
  3155  {

## vendor/bun/src/jsc/VirtualMachine.zig:2095-2135

- Full-file Git blob: 461906c6b3aaafae04772b69827f4beaca50b46e
- Full-file SHA-256: 8ce0ae2b45cec903d949ef52d0dcdeb2abb3b9bbe910aa644f99420cf8e9d667
- Full-file bytes: 162376
- Excerpt line span: 2095-2135
- Excerpt SHA-256: 6e6115052a1efe5966a300dec2826f44a7fc9d71fd3bada34a77eb64de677dd1

  2095                  globalThis.createAggregateError(
  2096                      errors,
  2097                      &ZigString.init(
  2098                          std.fmt.allocPrint(globalThis.allocator(), "{d} errors building \"{f}\"", .{
  2099                              errors.len,
  2100                              specifier,
  2101                          }) catch unreachable,
  2102                      ),
  2103                  ) catch |e| globalThis.takeException(e),
  2104              );
  2105          },
  2106      }
  2107  }
  2108
  2109  pub fn deinit(this: *VirtualMachine) void {
  2110      this.auto_killer.deinit();
  2111
  2112      if (source_code_printer) |print| {
  2113          print.getMutableBuffer().deinit();
  2114          print.ctx.written = &.{};
  2115      }
  2116      this.source_mappings.deinit();
  2117      if (this.rare_data) |rare_data| {
  2118          jsc.API.cron.CronJob.clearAllForVM(this, .teardown);
  2119          // Paired with rareData()'s registerRootRegion. Without this, every
  2120          // terminated Worker leaves a stale LSAN root entry pointing into a
  2121          // freed arena (harmless to the final leak verdict but accumulates one
  2122          // dead range per Worker for LSAN to scan).
  2123          bun.asan.unregisterRootRegion(rare_data, @sizeOf(jsc.RareData));
  2124          rare_data.deinit();
  2125      }
  2126      this.proxy_env_storage.deinit();
  2127      this.overridden_main.deinit();
  2128      this.entry_point.deinit();
  2129      this.has_terminated = true;
  2130  }
  2131
  2132  pub const ExceptionList = std.array_list.Managed(api.JsException);
  2133
  2134  pub fn printException(
  2135      this: *VirtualMachine,
