# Vendored JSC lifecycle supplemental source bundle (correction 5)

Exact product SHA: 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb

Exact product tree: cb964de8ab8162449fbe95959bf34d231570aa5c

The ordered lifecycle plan directly attaches complete `VirtualMachine.rs`, `JSGlobalObject.rs`, `VM.rs`, and `virtual_machine_exports.rs` bytes. This supplemental bundle binds their exact full-file identities plus the complete relevant C++/Zig termination, reset, teardown, and VM-call items. The C++ excerpt proves JSC__VM__deinit has an empty body; it cannot prove process death, containment drain, output joins, or retirement. Cooperative termination reset is therefore reusable only after the owner proves complete invocation and output drain independently.

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
