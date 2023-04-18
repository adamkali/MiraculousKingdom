export default function Layout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <div className="flex max-h-full w-3/5 flex-col justify-center rounded-3xl bg-slate-700/40 p-8 pb-6 backdrop-blur-lg">
            {children}
        </div>
    );
}
