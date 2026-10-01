! A sample.
module shapes
  implicit none
  private
  public :: point, area, largest

  integer, parameter :: limit = 10
  real, parameter :: pi = 3.14159

  type :: point
    real :: x = 0.0
    real :: y = 0.0
  end type point

contains

  pure function area(radius) result(a)
    real, intent(in) :: radius
    real :: a
    a = pi * radius**2
  end function area

  function largest(items) result(best)
    integer, intent(in) :: items(:)
    integer :: best, i
    best = items(1)
    do i = 2, size(items)
      if (items(i) > best) best = items(i)
    end do
  end function largest

end module shapes

program sample
  use shapes
  implicit none
  type(point) :: p
  integer :: i
  real, allocatable :: areas(:)

  p = point(3.0, 4.0)
  allocate(areas(3))
  do i = 1, 3
    areas(i) = area(real(i))
  end do

  if (sqrt(p%x**2 + p%y**2) > 1.0) then
    print *, 'long', largest([1, 2, 3])
  else
    print '(a)', 'short'
  end if

  select case (size(areas))
  case (0)
    stop 1
  case default
    write (*, '(3f8.2)') areas
  end select
  deallocate(areas)
end program sample
