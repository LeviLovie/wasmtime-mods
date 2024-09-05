@external("host", "host_func")
declare function host_func(a: i32): void;

export function hello(): void {
   host_func(3);
}
