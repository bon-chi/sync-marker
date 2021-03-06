use super::bookshelf::{FindByIsbn, InMemoryBooksRepository, Isbn};
use super::session::{FindUserId, InMemorySessionsRepository, SessionDigest};
use super::user::{FindById, InMemoryUsersRepository};
use actix::prelude::*;
use actix::{Addr, MailboxError};
use futures::{future, future::Either, Future};

impl InMemoryBookmarksRepository {
    pub fn new() -> Self {
        InMemoryBookmarksRepository(vec![Bookmark {
            id: 1,
            user_id: 1,
            book_id: 1,
            page_in_progress: 20,
        }])
    }
    fn find_by_user_id(&self, user_id: u32) -> Vec<Bookmark> {
        let mut bookmarks: Vec<Bookmark> = self
            .0
            .iter()
            .filter(|bookmark| bookmark.user_id == user_id)
            .map(|bookmark| bookmark.clone())
            .collect();
        bookmarks.sort_by_key(|bookmark| bookmark.book_id);
        bookmarks
    }
}

trait BookmarksRepository {
    // fn progress(
    //     &mut self,
    //     isbn: Isbn,
    //     page_in_progress: u16,
    //     session_digest: SessionDigest,
    //     sessions_repository: Addr<InMemorySessionsRepository>,
    //     users_repository: Addr<InMemoryUsersRepository>,
    //     books_repository: Addr<InMemoryBooksRepository>,
    //     // ) -> Box<dyn Future<Item = Result<(User, Book), String>, Error = actix::MailboxError>>;
    //     // ) -> Result<Result<(User, Book), String>, actix::MailboxError>;
    // ) -> Result<(), String>;
}

pub struct InMemoryBookmarksRepository(Vec<Bookmark>);

impl BookmarksRepository for InMemoryBookmarksRepository {
    // fn progress(
    //     &mut self,
    //     isbn: Isbn,
    //     page_in_progress: u16,
    //     session_digest: SessionDigest,
    //     sessions_repository: Addr<InMemorySessionsRepository>,
    //     users_repository: Addr<InMemoryUsersRepository>,
    //     books_repository: Addr<InMemoryBooksRepository>,
    //     // ) -> Result<(), ProgressBookmarkRepositoryError> {
    // ) -> Result<(), String> {
    //     // ) -> Result<Result<(User, Book), String>, actix::MailboxError> {
    //     // ) -> Box<dyn Future<Item = Result<(User, Book), String>, Error = actix::MailboxError>> {
    //     dbg!("6");
    //     let session = sessions_repository.send(FindUserId(session_digest));
    //     dbg!("7");
    //     // let user = session
    //     //     .then(|user_id| user_id.map(|user_id| user_id.map(|user_id| users_repository.send(FindById(user_id)))));
    //     let user = session.and_then(|user_id| {
    //         dbg!("11--bookmark");
    //         user_id.map(|user_id| {
    //             dbg!("12--bookmark");
    //             users_repository.send(FindById(user_id))
    //         })
    //     });
    //     dbg!("8");
    //     // let user = session.then(|user_id| match user_id {
    //     //     Some(user_id) => users_repository.send(FindById(user_id)),
    //     //     None => futures::future::err(""),
    //     // });
    //     // let _user = match session {
    //     //     Ok(session) => {}
    //     //     Err(err) => {}
    //     //
    //     // }
    //     // let user = user.then(|user_result| match user_result {
    //     //     Ok(user_result) => match user_result {
    //     //         Some(user_result) => Ok(user_result),
    //     //         None => panic!(""),
    //     //     },
    //     //     Err(err) => panic!(""),
    //     // });
    //     // let user = session.and_then(|user_id| match user_id {
    //     //     Some(user_id) => users_repository.send(FindById(user_id)),
    //     //     None => panic!(""),
    //     // });
    //     // let user_res = session.and_then(|user_id| match user_id {
    //     //     Some(user_id) => Ok(users_repository.send(FindById(user_id))),
    //     //     None => Err("no"),
    //     // });
    //     // let user_future = users_repository.send(FindById)
    //     let book = books_repository.send(FindByIsbn(isbn));
    //     dbg!("8");
    //     let user_and_book = user.join(book);
    //     dbg!("10");
    //
    //     // let user_and_book = user_and_book.map(|(user, book)| {
    //     //     let user = match user {
    //     //         Some(user) => match user {
    //     //             Some(user) => user,
    //     //             None => {
    //     //                 return Err(ProgressBookmarkRepositoryError::UserNotFoundError.to_string());
    //     //             }
    //     //         },
    //     //         None => {
    //     //             return Err(ProgressBookmarkRepositoryError::UserNotFoundError.to_string());
    //     //         }
    //     //     };
    //     //     if book.is_none() {
    //     //         return Err(ProgressBookmarkRepositoryError::BookNotFoundError(isbn).to_string());
    //     //     }
    //     //     Ok((user, book.unwrap()))
    //     // });
    //
    //     // let (user, book) = user_and_book.unwrap().unwrap();
    //     // if let Some(bookmark) = self
    //     //     .0
    //     //     .iter_mut()
    //     //     .find(|bookmark| bookmark.user_id == user.id && bookmark.book_id == book.id)
    //     // {
    //     //     bookmark.page_in_progress = page_in_progress
    //     // } else {
    //     //     let latest_bookmark = self.0.iter().max_by_key(|bookmark| bookmark.id);
    //     //     let bookmark = if let Some(latest_bookmark) = latest_bookmark {
    //     //         Bookmark {
    //     //             id: latest_bookmark.id + 1,
    //     //             user_id: user.id,
    //     //             book_id: book.id,
    //     //             page_in_progress,
    //     //         }
    //     //     } else {
    //     //         Bookmark {
    //     //             id: 1,
    //     //             user_id: user.id,
    //     //             book_id: book.id,
    //     //             page_in_progress,
    //     //         }
    //     //     };
    //     //     self.0.push(bookmark)
    //     // }
    //     //
    //     // return user_and_book.boxed();
    //
    //     // let data = user_and_book.map(|(user, book)| {
    //     //     dbg!("13--bookmark");
    //     //     let user = match user {
    //     //         Some(user) => match user {
    //     //             Some(user) => user,
    //     //             None => {
    //     //                 return Err(ProgressBookmarkRepositoryError::UserNotFoundError.to_string());
    //     //             }
    //     //         },
    //     //         None => {
    //     //             return Err(ProgressBookmarkRepositoryError::UserNotFoundError.to_string());
    //     //         }
    //     //     };
    //     //     dbg!("14--bookmark");
    //     //     let book = if let Some(book) = book {
    //     //         book
    //     //     } else {
    //     //         return Err(ProgressBookmarkRepositoryError::BookNotFoundError(isbn).to_string());
    //     //     };
    //     //     dbg!("15--bookmark");
    //     //     if let Some(bookmark) = self
    //     //         .0
    //     //         .iter_mut()
    //     //         .find(|bookmark| bookmark.user_id == user.id && bookmark.book_id == book.id)
    //     //     {
    //     //         bookmark.page_in_progress = page_in_progress
    //     //     } else {
    //     //         let latest_bookmark = self.0.iter().max_by_key(|bookmark| bookmark.id);
    //     //         let bookmark = if let Some(latest_bookmark) = latest_bookmark {
    //     //             Bookmark {
    //     //                 id: latest_bookmark.id + 1,
    //     //                 user_id: user.id,
    //     //                 book_id: book.id,
    //     //                 page_in_progress,
    //     //             }
    //     //         } else {
    //     //             Bookmark {
    //     //                 id: 1,
    //     //                 user_id: user.id,
    //     //                 book_id: book.id,
    //     //                 page_in_progress,
    //     //             }
    //     //         };
    //     //         self.0.push(bookmark)
    //     //     }
    //     //     dbg!("16--bookmark");
    //     //
    //     //     Ok((user.clone(), book.clone()))
    //     // });
    //     // Box::new(data)
    //
    //     // // let mut core = tokio::runtime::Runtime::new().unwrap();
    //     // let mut rt = tokio::runtime::current_thread::Runtime::new().expect("new rt");
    //     // let hoge = rt.block_on(data);
    //     // // let hoge = core.block_on(data);
    //     // match hoge {
    //     //     Ok(data) => match data {
    //     //         Ok(data) => Ok(()),
    //     //         Err(err) => Err("foo".to_string()),
    //     //     },
    //     //     Err(err) => Err("bar".to_string()),
    //     // }
    //     // Arbiter::spawn(FutureOk(1).map(|integ| println!("ok")));
    //     Arbiter::spawn_fn(move || {
    //         let session = sessions_repository.send(FindUserId(session_digest));
    //         let user = session.and_then(|user_id| user_id.map(|user_id| users_repository.send(FindById(user_id))));
    //         let book = books_repository.send(FindByIsbn(isbn));
    //         let user_and_book = user.join(book);
    //         let user_and_book = user_and_book.wait().map(|(user, book)| {
    //             let user = match user {
    //                 Some(user) => match user {
    //                     Some(user) => user,
    //                     None => {
    //                         return Err(ProgressBookmarkRepositoryError::UserNotFoundError.to_string());
    //                     }
    //                 },
    //                 None => {
    //                     return Err(ProgressBookmarkRepositoryError::UserNotFoundError.to_string());
    //                 }
    //             };
    //             if book.is_none() {
    //                 return Err(ProgressBookmarkRepositoryError::BookNotFoundError(isbn).to_string());
    //             }
    //             Ok((user, book.unwrap()))
    //         });
    //
    //         let (user, book) = user_and_book.unwrap().unwrap();
    //         let cloned = self.clone();
    //         if let Some(bookmark) = self
    //             .0
    //             .iter_mut()
    //             .find(|bookmark| bookmark.user_id == user.id && bookmark.book_id == book.id)
    //         {
    //             //     bookmark.page_in_progress = page_in_progress
    //         } else {
    //             //     let latest_bookmark = self.0.iter().max_by_key(|bookmark| bookmark.id);
    //             //     let bookmark = if let Some(latest_bookmark) = latest_bookmark {
    //             //         Bookmark {
    //             //             id: latest_bookmark.id + 1,
    //             //             user_id: user.id,
    //             //             book_id: book.id,
    //             //             page_in_progress,
    //             //         }
    //             //     } else {
    //             //         Bookmark {
    //             //             id: 1,
    //             //             user_id: user.id,
    //             //             book_id: book.id,
    //             //             page_in_progress,
    //             //         }
    //             //     };
    //             //     self.0.push(bookmark)
    //         }
    //
    //         // let user = match user {
    //         //     Some(user) => match user {
    //         //         Some(user) => user,
    //         //         None => {
    //         //             return Err(ProgressBookmarkRepositoryError::UserNotFoundError.to_string());
    //         //         }
    //         //     },
    //         //     None => {
    //         //         return Err(ProgressBookmarkRepositoryError::UserNotFoundError.to_string());
    //         //     }
    //         // };
    //         // let book = if let Some(book) = book {
    //         //     book
    //         // } else {
    //         //     return Err(ProgressBookmarkRepositoryError::BookNotFoundError(isbn).to_string());
    //         // };
    //         // if let Some(bookmark) = self
    //         //     .0
    //         //     .iter_mut()
    //         //     .find(|bookmark| bookmark.user_id == user.id && bookmark.book_id == book.id)
    //         // {
    //         //     bookmark.page_in_progress = page_in_progress
    //         // } else {
    //         //     let latest_bookmark = self.0.iter().max_by_key(|bookmark| bookmark.id);
    //         //     let bookmark = if let Some(latest_bookmark) = latest_bookmark {
    //         //         Bookmark {
    //         //             id: latest_bookmark.id + 1,
    //         //             user_id: user.id,
    //         //             book_id: book.id,
    //         //             page_in_progress,
    //         //         }
    //         //     } else {
    //         //         Bookmark {
    //         //             id: 1,
    //         //             user_id: user.id,
    //         //             book_id: book.id,
    //         //             page_in_progress,
    //         //         }
    //         //     };
    //         //     self.0.push(bookmark)
    //         // }
    //         //
    //         // Ok((user.clone(), book.clone()))
    //         Ok(())
    //     });
    //     // hoge.clone()
    //     // println!("progress is {:?}", self.0);
    //     Ok(())
    // }
}

#[derive(Fail, Debug)]
pub enum ProgressBookmarkRepositoryError {
    #[fail(display = "Session Not Found")]
    SessionNotFoundError,
    #[fail(display = "User Not Found")]
    UserNotFoundError,
    #[fail(display = "Book Not Found")]
    BookNotFoundError(Isbn),
    #[fail(display = "page_cuont max is {}, but entered {}", _1, _0)]
    PageCountOverFlowError(u16, i32),
    #[fail(display = "Acctor Error")]
    ActorError(MailboxError),
}
#[derive(Debug, Clone)]
pub struct Bookmark {
    pub id: u32,
    user_id: u32,
    pub book_id: u32,
    pub page_in_progress: u16,
}
impl Actor for InMemoryBookmarksRepository {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("BookmarksRepository Actor is alive");
    }
    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("BookmarksRepository Actor is stopped");
    }
}

// message
pub struct Progress {
    pub isbn: Isbn,
    pub page_in_progress: u16,
    pub session_digest: SessionDigest,
    pub sessions_repository: Addr<InMemorySessionsRepository>,
    pub users_repository: Addr<InMemoryUsersRepository>,
    pub books_repository: Addr<InMemoryBooksRepository>,
}

impl Message for Progress {
    type Result = Result<(u32, u16), ProgressBookmarkRepositoryError>;
    // type Result = Result<BookAndLocation, ReqwestError>;
}

impl Handler<Progress> for InMemoryBookmarksRepository {
    // type Result = Result<(), ProgressBookmarkRepositoryError>;
    type Result = ResponseActFuture<Self, (u32, u16), ProgressBookmarkRepositoryError>;
    fn handle(&mut self, msg: Progress, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            Progress {
                isbn,
                page_in_progress,
                session_digest,
                sessions_repository,
                users_repository,
                books_repository,
            } => {
                let session = sessions_repository
                    .send(FindUserId(session_digest))
                    .map_err(|e| ProgressBookmarkRepositoryError::ActorError(e));
                // .map_err(|_| "e");
                let user = session.and_then(move |user_id| match user_id {
                    Some(user_id) => Either::A(
                        users_repository
                            .send(FindById(user_id))
                            .map_err(|e| ProgressBookmarkRepositoryError::ActorError(e)),
                    ),
                    None => Either::B(future::err(ProgressBookmarkRepositoryError::SessionNotFoundError)),
                });
                // let user = session.then(move |user_id| match user_id {
                //     Ok(user_id) => match user_id {
                //         Some(user_id) => Either::A(users_repository.send(FindById(user_id))),
                //         None => Either::B(future::err(ProgressBookmarkRepositoryError::SessionNotFoundError)),
                //     },
                //     Err(e) => Either::B(future::err(ProgressBookmarkRepositoryError::SessionNotFoundError)),
                // });
                let book = books_repository
                    .send(FindByIsbn(isbn))
                    .map_err(|e| ProgressBookmarkRepositoryError::ActorError(e));
                let user_book = user.join(book).into_actor(self).then(move |res, act, ctx| {
                    match res {
                        Ok((user, book)) => match (user, book) {
                            (Some(user), Some(book)) => {
                                let mut ret: (u32, u16);
                                if page_in_progress as i32 > book.page_count() {
                                    return fut::err(ProgressBookmarkRepositoryError::PageCountOverFlowError(
                                        page_in_progress,
                                        book.page_count(),
                                    ));
                                }
                                let mut ret: (u32, u16);
                                if let Some(bookmark) = act
                                    .0
                                    .iter_mut()
                                    .find(|bookmark| bookmark.user_id == user.id && bookmark.book_id == book.id)
                                {
                                    bookmark.page_in_progress = page_in_progress;
                                    ret = (bookmark.id, page_in_progress);
                                } else {
                                    let latest_bookmark = act.0.iter().max_by_key(|bookmark| bookmark.id);
                                    let bookmark = if let Some(latest_bookmark) = latest_bookmark {
                                        Bookmark {
                                            id: latest_bookmark.id + 1,
                                            user_id: user.id,
                                            book_id: book.id,
                                            page_in_progress,
                                        }
                                    } else {
                                        Bookmark {
                                            id: 1,
                                            user_id: user.id,
                                            book_id: book.id,
                                            page_in_progress,
                                        }
                                    };
                                    ret = (bookmark.id, page_in_progress);
                                    act.0.push(bookmark)
                                }
                                println!("{:?}", act.0);
                                fut::ok(ret)
                            }
                            // (None, _) => fut::err(()),
                            (None, _) => fut::err(ProgressBookmarkRepositoryError::UserNotFoundError),
                            (_, None) => fut::err(ProgressBookmarkRepositoryError::BookNotFoundError(isbn)),
                        },
                        Err(err) => {
                            ctx.stop();
                            fut::err(err)
                            // fut::ok(())
                        }
                    }
                    // println!("inner");
                    // fut::ok(())
                });
                return Box::new(user_book);
                // user_book.wait(ctx);
                // user_book.wait();
            }
        }
        // let session = msg.sessions_repository.send(FindUserId(msg.session_digest));
        // let user = session.and_then(move |user_id| user_id.map(|user_id| msg.users_repository.send(FindById(user_id))));
        // let book = msg.books_repository.send(FindByIsbn(isbn));
        // let user_and_book = user
        //     .join(book)
        //     .into_actor(self)
        //     .then(|res, act, ctx| fut::ok(()))
        //     .wait(ctx);
        // let mut my_user_id: Option<u32> = None;
        // let session1 = msg
        //     .sessions_repository
        //     .send(FindUserId(msg.session_digest))
        //     .into_actor(self)
        //     .then(move |res, act, ctx| {
        //         println!("inner");
        //         let hoge = match res {
        //             Ok(res) => match res {
        //                 Some(user_id) => {
        //                     println!("inner: {}", user_id);
        //                     my_user_id = Some(user_id)
        //                 }
        //                 None => {
        //                     println!("inner 2");
        //                 }
        //             },
        //             _ => {
        //                 println!("inner 1");
        //                 ctx.stop()
        //             }
        //         };
        //         fut::ok(())
        //     })
        //     .wait(ctx);
        // println!("{:?}", my_user_id);
        // let user = session.and_then(|user_id| {
        //     dbg!("11--bookmark");
        //     user_id.map(|user_id| {
        //         dbg!("12--bookmark");
        //         users_repository.send(FindById(user_id))
        //     })
        // });
        // self.0.push(Bookmark {
        //     id: 1,
        //     user_id: 1,
        //     book_id: 1,
        //     page_in_progress: 1,
        // });
        // Ok(())
    }
}
#[derive(Debug)]
pub struct Never;

pub struct FindByUserId(pub u32);
impl Message for FindByUserId {
    type Result = Result<Vec<Bookmark>, Never>;
}
impl Handler<FindByUserId> for InMemoryBookmarksRepository {
    type Result = Result<Vec<Bookmark>, Never>;
    fn handle(&mut self, msg: FindByUserId, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(self.find_by_user_id(msg.0))
    }
}
