'use client'
import { Command, CommandEmpty, CommandList, CommandInput } from '@/components/ui/command'
import { useState } from 'react'

export const SearchPage = () => {
  const [open, setOpen] = useState<boolean>(false)
  const handleValueChange = (value: string) => {
    setOpen(!!value)
  }
  return (
    <div style={{ background: 'linear-gradient(to right, #d4e1f5, #ffcbb9, #ffe29e)' }} className="h-full flex flex-col justify-center items-center">
      <div className="flex flex-col text-center p-10">
        <h1 style={{ textShadow: '0 2px 4px rgba(0, 0, 0, 0.2)' }} className="mb-5 text-white text-5xl">
          Let's Rank It.
        </h1>
        <Command className="w-96 drop-shadow">
          <CommandInput placeholder="Search" onValueChange={handleValueChange} />
          <CommandList>{open && <CommandEmpty>No results found.</CommandEmpty>}</CommandList>
        </Command>
      </div>
    </div>
  )
}
