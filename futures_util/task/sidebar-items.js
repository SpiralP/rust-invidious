initSidebarItems({"enum":[["Poll","Indicates whether a value is available or if the current task has been scheduled to receive a wakeup instead."]],"fn":[["noop_waker","Create a new [`Waker`] which does nothing when `wake()` is called on it."]],"struct":[["AtomicWaker","A synchronization primitive for task wakeup."],["Context","The `Context` of an asynchronous task."],["FutureObj","A custom trait object for polling futures, roughly akin to `Box<dyn Future<Output = T> + Send + 'a>`."],["LocalFutureObj","A custom trait object for polling futures, roughly akin to `Box<dyn Future<Output = T> + 'a>`."],["RawWaker","A `RawWaker` allows the implementor of a task executor to create a `Waker` which provides customized wakeup behavior."],["RawWakerVTable","A virtual function pointer table (vtable) that specifies the behavior of a `RawWaker`."],["SpawnError","An error that occurred during spawning."],["Waker","A `Waker` is a handle for waking up a task by notifying its executor that it is ready to be run."]],"trait":[["LocalSpawn","The `LocalSpawn` is similar to [`Spawn`], but allows spawning futures that don't implement `Send`."],["LocalSpawnExt","Extension trait for `LocalSpawn`."],["Spawn","The `Spawn` trait allows for pushing futures onto an executor that will run them to completion."],["SpawnExt","Extension trait for `Spawn`."],["UnsafeFutureObj","A custom implementation of a future trait object for `FutureObj`, providing a vtable with drop support."]]});