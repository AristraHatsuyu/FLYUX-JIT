F>等级权限(level){
  if(level="admin"){
    print("👑 超级管理员：拥有全部权限")
  }(level="staff"){
    print("🔧 内部员工：可查看和编辑数据")
  }(level="guest"){
    print("🪪 访客：仅可浏览公共信息")
  }{
    print("❓ 未知身份")
  }
}

F>等级权限标准(level){
  if(level="admin"){
    print("👑 超级管理员：拥有全部权限")
  } elif(level="staff"){
    print("🔧 内部员工：可查看和编辑数据")
  } elif(level="guest"){
    print("🪪 访客：仅可浏览公共信息")
  } else {
    print("❓ 未知身份")
  }
}

F>main(){
  等级权限("admin")
  等级权限("guest")
  等级权限标准("staff")
  等级权限标准("nobody")
}