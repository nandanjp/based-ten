import { prisma } from '../../db/db'
import { FollowsFileType } from '../../db/seed/data-schemas'
import {
  CreateGroupType,
  CreateListType,
  CreateUserType,
  RemoveListLikeType,
  UnfollowUserType,
  UserGroupsParamsType,
  UserListType,
  UserParamsType
} from './users.schema'

export const getAllUsers = async () => {
  return await prisma.users.findMany({})
}

export const getUser = async (params: UserParamsType) => {
  return await prisma.users.findUnique({
    where: {
      email: params.email
    }
  })
}

export const getAllUsersLists = async (params: UserParamsType) => {
  return await prisma.lists.findMany({
    where: {
      email: params.email
    }
  })
}

export const getUserList = async (params: UserListType) => {
  return await prisma.users.findUnique({
    where: {
      email: params.email
    },
    include: {
      lists: {
        where: {
          listname: params.list_name
        }
      }
    }
  })
}

export const getAllLikedListsOfUser = async (params: UserParamsType) => {
  return await prisma.lists.findMany({
    where: {
      email: {
        equals: params.email
      }
    },
    include: {
      likes: {}
    }
  })
}

export const getAllGroupsOfUser = async (params: UserParamsType) => {
  return await prisma.users.findUnique({
    where: {
      email: params.email
    },
    include: {
      groups: {}
    }
  })
}

export const getGroupOfUser = async (params: UserGroupsParamsType) => {
  return await prisma.users.findUnique({
    where: {
      email: params.email
    },
    include: {
      groups: {
        where: {
          gid: parseInt(params.gid)
        }
      }
    }
  })
}

export const getAllGroupMembersOfUserGroup = async (params: UserGroupsParamsType) => {
  return await prisma.users.findUnique({
    where: {
      email: params.email
    },
    include: {
      groupmembers: {
        where: {
          gid: parseInt(params.gid)
        }
      }
    }
  })
}

export const getAllFollowersOfUser = async (params: UserParamsType) => {
  return await prisma.users.findUnique({
    where: {
      email: params.email
    },
    include: {
      follows_follows_followeremailTousers: {}
    }
  })
}

export const getAllFollowingOfUser = async (params: UserParamsType) => {
  return await prisma.users.findUnique({
    where: {
      email: params.email
    },
    include: {
      follows_follows_followingemailTousers: {}
    }
  })
}

export const createUser = async (body: CreateUserType) => {
  return await prisma.users.create({
    data: {
      email: body.email,
      displayname: body.display_name,
      userpassword: body.user_password
    }
  })
}

export const createUserList = async (params: UserParamsType, body: CreateListType) => {
  const list = await prisma.lists.create({
    data: {
      email: params.email,
      listname: body.list_name,
      listtype: body.list_type
    }
  })
  for (const item of body.items) {
    await prisma.listitems.create({
      data: {
        itemid: parseInt(item.item_id),
        rankinginlist: item.ranking_in_list,
        listname: body.list_name,
        email: params.email
      }
    })
  }
  return await prisma.lists.findUnique({
    where: {
      listname: list.listname,
      email: list.email,
      email_listname: {
        listname: list.listname,
        email: list.email
      }
    },
    include: {
      listitems: {}
    }
  })
}

export const createUserGroup = async (params: UserParamsType, body: CreateGroupType) => {
  return await prisma.groups.create({
    data: {
      groupname: body.group_name,
      ownedby: params.email
    }
  })
}

export const createFollowing = async (params: UserParamsType, body: FollowsFileType) => {
  return await prisma.follows.create({
    data: {
      followeremail: params.email,
      followingemail: body.following_email
    }
  })
}

export const deleteUser = async (params: UserParamsType) => {
  return await prisma.users.delete({
    where: {
      email: params.email
    }
  })
}

export const deleteUserFollow = async (params: UnfollowUserType) => {
  return await prisma.follows.delete({
    where: {
      followeremail_followingemail: {
        followeremail: params.follower_email,
        followingemail: params.following_email
      }
    }
  })
}

export const deleteUserList = async (params: UserListType) => {
  return await prisma.users.delete({
    where: {
      email: params.email
    },
    include: {
      lists: {
        where: {
          listname: params.list_name
        }
      }
    }
  })
}

export const deleteUserLike = async (params: RemoveListLikeType) => {
  return await prisma.likes.delete({
    where: {
      likeremail_listname: {
        likeremail: params.liker_email,
        listname: params.list_name
      }
    }
  })
}
