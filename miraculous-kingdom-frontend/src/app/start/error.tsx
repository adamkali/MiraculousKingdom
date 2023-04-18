"use client";

export default function Error({
    error,
    reset,
}: {
    error: Error,
    reset: () => void,
}) {
    return (
        <div className="text-purple-600">
            {error.message}
            <button onClick={reset}>
                Hi
            </button>
        </div>
    )
}
