"""
Try to generate events as quickly as possible with the nexmark module.
"""

import nexmark
import time
import threading
import multiprocessing

def process_worker(duration):
    config = nexmark.Config()
    gen = nexmark.EventGenerator(config)
    local_count = 0
    start = time.time()
    while time.time() - start < duration:
        next(gen)
        local_count += 1
    return local_count

def main():
    # Single producer
    config = nexmark.Config()
    gen = nexmark.EventGenerator(config)

    event_count = 0
    start_time = time.time()
    duration = 10  # seconds to measure throughput

    for event in gen:
        event_count += 1
        if time.time() - start_time >= duration:
            break

    elapsed = time.time() - start_time
    throughput = event_count / elapsed
    print(f"Generated {event_count} events in {elapsed:.2f} seconds ({throughput:.2f} events/sec)")

    # Thread pool of 4 producers
    num_threads = 4
    event_counts = [0] * num_threads

    def thread_worker(idx):
        config = nexmark.Config()
        gen = nexmark.EventGenerator(config)
        local_count = 0
        thread_start = time.time()
        while time.time() - thread_start < duration:
            next(gen)
            local_count += 1
        event_counts[idx] = local_count

    threads = []
    for i in range(num_threads):
        t = threading.Thread(target=thread_worker, args=(i,))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()

    total_events = sum(event_counts)
    print(f"Generated {total_events} events with {num_threads} generator threads in {duration} seconds ({total_events/duration:.2f} events/sec)")

    # Process pool of 4 producers
    num_processes = 4

    with multiprocessing.Pool(num_processes) as pool:
        time.sleep(1)  # Allow processes to start up
        results = pool.map(process_worker, [duration] * num_processes)

    total_events = sum(results)
    print(f"Generated {total_events} events with {num_processes} generator processes in {duration} seconds ({total_events/duration:.2f} events/sec)")

if __name__ == "__main__":
    main()
