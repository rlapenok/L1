struct OldRepo;
trait ForOldRepo{
    fn do_it(&self){}

}
impl ForOldRepo for OldRepo{
    fn do_it(&self){}
}
struct RepoAdapter<T>
    where T:ForOldRepo
{
    repo:T
}

trait ForNewRepo {
   fn  something(&self){}
}
impl <T> ForNewRepo for RepoAdapter<T>
    where T:ForOldRepo
{
    fn  something(&self) {
        self.repo.do_it()
    }


}


fn main() {
    let repo_adapter=RepoAdapter{repo:OldRepo};
    repo_adapter.something();
}
