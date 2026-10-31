// The offline loader returns completed tasks. No scheduling or networking is emulated.
#pragma once
#include <exception>
#include <optional>
#include <utility>
namespace concurrency {
template<class T> class task {
    std::optional<T> value_;
    std::exception_ptr error_;
public:
    task() = default;
    explicit task(T value) : value_(std::move(value)) {}
    explicit task(std::exception_ptr error) : error_(error) {}
    T get() const {
        if (error_) std::rethrow_exception(error_);
        if (!value_) throw std::logic_error("Uninitialized offline task");
        return *value_;
    }
    template<class F> auto then(F&& f) const -> task<decltype(f(std::declval<T>()))> {
        using R = decltype(f(std::declval<T>()));
        try { return task<R>(f(get())); }
        catch (...) { return task<R>(std::current_exception()); }
    }
};
template<class T> task<T> task_from_result(T value) { return task<T>(std::move(value)); }
}
