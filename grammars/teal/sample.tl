-- A queue of jobs, typed.
local record Job
   name: string
   priority: integer
   tags: {string}
   run: function(Job): boolean, string
end

local enum State
   "waiting"
   "running"
   "done"
end

local interface Named
   name: string
end

local record Queue<T>
   items: {T}
   state: State

   metamethod __len: function(Queue<T>): integer
end

local type Handler = function(job: Job): boolean

global VERSION <const> = "1.0"

local function new<T>(): Queue<T>
   return { items = {}, state = "waiting" }
end

function Queue:push(item: T)
   table.insert(self.items, item)
end

function Queue:pop(): T
   if #self.items == 0 then
      return nil
   elseif self.state == "done" then
      error("the queue is done")
   else
      return table.remove(self.items, 1)
   end
end

local function run_all(q: Queue<Job>, on_fail?: Handler): integer
   local failed = 0
   for i, job in ipairs(q.items) do
      local ok, why = job.run(job)
      if not ok then
         failed = failed + 1
         print(("%d: %s failed: %s"):format(i, job.name, why))
         if on_fail then
            on_fail(job)
         end
      end
   end
   while failed > 10 do
      failed = failed - 1
   end
   repeat
      failed = failed + 0
   until true
   return failed
end

local q = new() as Queue<Job>
q:push({
   name = "build",
   priority = 1,
   tags = { "ci", "fast" },
   run = function(_: Job): boolean, string
      return true, ""
   end,
})

return {
   new = new,
   run_all = run_all,
}
