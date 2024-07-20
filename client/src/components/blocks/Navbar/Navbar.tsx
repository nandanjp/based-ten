import Link from 'next/link';

const Navbar = () => {
  return (
    <div className="flex justify-between w-full bg-gray-600 text-white text-xl">
      <div className="flex gap-2">
        <Link href="/" className="hover:bg-gray-400 p-3">
          Home
        </Link>
        <Link href="/explore" className="hover:bg-gray-400 p-3">
          Explore
        </Link>
      </div>
      <div className="flex gap-2">
        <Link href="/login" className="hover:bg-gray-400 p-3">
          Log In
        </Link>
        <Link href="/signup" className="hover:bg-gray-400 p-3">
          Sign Up
        </Link>
      </div>
    </div>
  );
};

export default Navbar;
