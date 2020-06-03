#include <seastar/core/app-template.hh>
#include <seastar/core/sleep.hh>
#include <seastar/core/fstream.hh>
#include <seastar/core/seastar.hh>
#include <seastar/core/future-util.hh>
#include <iostream>
#include <chrono>

#include "seastar.h"

void seastar_app_template_run(void* ctx, int argc, char** argv) {
    seastar::app_template app;
    app.run(argc, argv, [ctx = std::move(ctx)] {
        seastar_rs_closure_void(ctx);
        using namespace std::chrono_literals;
        return seastar::sleep(3s).then([] {
            std::cout << "Done.\n";
        });
    });
}

void seastar_spawn(void* ctx) {
    seastar::make_ready_future<>().then([ctx = std::move(ctx)] {
        seastar_rs_closure_void(ctx);
    });
}

void seastar_sleep(void* ctx, unsigned long ns) {
    seastar::sleep(std::chrono::nanoseconds(ns)).then([ctx = std::move(ctx)] {
        seastar_rs_closure_void(ctx);
    });
}

void seastar_file_size(void* ctx, char* fname) {
    seastar::file_size(fname).then([ctx = std::move(ctx)] (uint64_t size) {
        seastar_rs_closure_u64(ctx, size);
    });
}

void seastar_open_file_dma(void* ctx, char* fname, int flags) {
    seastar::open_file_dma(fname, static_cast<seastar::open_flags>(flags))
        .then([ctx = std::move(ctx)] (seastar::file f) {
            seastar_rs_closure_voidptr(ctx, static_cast<void*>(new seastar::file(f)));
        });
}

// void seastar_ostream_write(void* ctx, void* optr) {
//     static_cast<seastar::ostream*>(optr)->write()
//         .then([ctx = std::move(ctx)] {
//             seastar_rs_closure_void(ctx);
//         });
// }

void seastar_output_stream_char_close(void* ctx, void* fptr) {
    static_cast<seastar::output_stream<char>*>(fptr)->close()
        .then([ctx = std::move(ctx)] {
            seastar_rs_closure_void(ctx);
        });
}




// extern "C" {
    // void seastar_rs_main();
    // void seastar_rs_closure_void(void* ctx);
    // void seastar_rs_closure_u32(void* ctx, int32_t);
    // void seastar_rs_closure_u64(void* ctx, uint64_t);
    // void seastar_rs_closure_voidptr(void* ctx, void*);

    // void seastar_sleep(unsigned long ns, void* ctx) {
    //     seastar::sleep(std::chrono::nanoseconds(ns)).then([ctx = std::move(ctx)] {
    //         seastar_rs_closure_void(ctx);
    //     });
    // }

    // void seastar_spawn(void* ctx) {
    //     seastar::make_ready_future<>().then([ctx = std::move(ctx)] {
    //         seastar_rs_closure_void(ctx);
    //     });
    // }

    // void seastar_file_size(const char* fname, void* ctx) {
    //     seastar::file_size(fname).then([ctx = std::move(ctx)] (uint64_t size) {
    //         seastar_rs_closure_u64(ctx, size);
    //     });
    // }

    // void seastar_open_file_dma(const char* fname, int flags, void* ctx) {
    //     // seastar::open_file_dma(fname, (seastar::open_flags)flags).then([ctx = std::move(ctx)] (seastar::file f) {
    //     //     // seastar_rs_closure_voidptr(ctx, (void*));
    //     // });
    // }

    // void seastar_new(int size) {
    //     new char[size];
    // }

    // void seastar_delete(void* ptr) {
    //     delete ptr;
    // }


// using namespace std::chrono_literals;

// int main(int argc, char** argv) {
//     seastar::app_template app;
//     app.run(argc, argv, [] {
//         seastar_rs_main();
//         return seastar::sleep(3s).then([] {
//             std::cout << "Done.\n";
//         });
//     });
// }


// typedef void (*rsfuture)(void*);

// void done() {
//     std::cout << "Done.\n";
// }

// seastar::future<void*> seastar_make_ready_future(void* data) {
//     return seastar::make_ready_future<void*>(data);
// }


// extern "C" void* seastar_sleep2(unsigned long ns) {
//     // seastar::make_ready_future<>().available
// }

// extern "C" void seastar_sleep(void* ctx, unsigned long ns) {
//     seastar::sleep(std::chrono::nanoseconds(ns)).then([ctx = std::move(ctx)] {
//         resume_closure(ctx);
//     });
// }

// extern "C" void seastar_spawn(void* task) {
//     seastar::make_ready_future<void*>(task).then([] (void* task) {
//         seastar_rs_task_poll(task);
//     });
// }

// extern "C" void seastar_open_file_dma(void* ctx_cond, void* ctx_body) {
//     seastar::make_ready_future<>().then([ctx = std::move(ctx_cond)] {
//         resume_task(ctx);
//     });
// }

// extern "C" void* seastar_make_sleep_future(void* task, unsigned long ns) {
//     seastar::sleep(std::chrono::nanoseconds(ns)).then([task = std::move(task)] {
//         seastar_rs_task_poll(task);
//     });
//     // auto fut = seastar::sleep(std::chrono::nanoseconds(ns));
//     // seastar::when_all(std::move(fut)).then([] (auto tup) {
//     //     seastar_rs_future_poll(rs_main);
//     // });

//     // fut.then([fut = std::move(fut)] {
//     //     std::cout << "GNA: " << fut.available() << "\n";
//     // });
//     // seastar::future<>* fptr = new seastar::future(std::move(fut));
//     // (*fptr).then([fptr] {
//     //     std::cout << "GNA: " << (*fptr).available() << "\n";
//     //     // seastar_rs_future_poll(rs_main);
//     // });
//     // seastar::future<>* fptr = new seastar::future(fut);
//     return (void*)NULL;
// }

// extern "C" int is_future_ready(void* future_ptr) {
//     auto fut = (seastar::future<>*)(future_ptr);
//     return (*fut).available();
// }
