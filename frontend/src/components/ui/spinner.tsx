
export default function Spinner() {
    return (
        <div className="flex items-center justify-center p-4">
            <div className="relative">
                <div className="w-12 h-12 rounded-full border-4 border-gray-600 border-t-transparent animate-spin"></div>
                <div className="absolute inset-0 w-12 h-12 rounded-full border-4 border-transparent border-t-cyan-400 animate-spin" 
                     style={{ animationDuration: '0.8s' }}></div>
            </div>
        </div>
    );
}