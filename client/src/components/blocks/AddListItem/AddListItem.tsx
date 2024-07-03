import { Button } from '@/components/ui/button'
import { Plus } from 'lucide-react'

export const AddListItem = () => {
  return (
    <div className="">
      <Button className="flex gap-4 justify-normal rounded-2xl px-8 py-10 w-80 bg-gray-600 text-white hover:bg-gray-700">
        <Plus className="border-dashed border-2 rounded-sm" />
        <div className="flex flex-col gap-2 items-center justify-center">
          <h1 className="font-semibold italic text-base">Choose an item...</h1>
        </div>
      </Button>
    </div>
  )
}
