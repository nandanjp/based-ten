import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

const LoginPage = () => {
  return (
    <div className="w-screen h-screen flex flex-col items-center justify-center gap-2">
      <div className="flex flex-col gap-4">
        <Input
          className="border-gray-900 rounded-2xl w-auto p-4 focus-visible:ring-0 focus-visible:ring-offset-0 placeholder-black text-lg font-bold placeholder-opacity-20"
          type="email"
          placeholder="Email"
        />
        <Input
          className="border-gray-900 rounded-2xl w-auto p-4 focus-visible:ring-0 focus-visible:ring-offset-0 placeholder-black text-lg font-bold placeholder-opacity-20"
          type="password"
          placeholder="Password"
        />
        <div className="flex gap-2">
          <Button className="bg-primary hover:bg-gray-700 rounded-xl gap-2 py-5 w-1/2">
            Sign Up
          </Button>
          <Button className="bg-primary hover:bg-gray-700 rounded-xl gap-2 py-5 w-1/2">
            Login
          </Button>
        </div>
      </div>
    </div>
  );
};

export default LoginPage;
